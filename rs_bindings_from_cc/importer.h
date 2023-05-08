// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_

#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/log/die_if_null.h"
#include "common/status_macros.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/importers/class_template.h"
#include "rs_bindings_from_cc/importers/cxx_record.h"
#include "rs_bindings_from_cc/importers/enum.h"
#include "rs_bindings_from_cc/importers/friend.h"
#include "rs_bindings_from_cc/importers/function.h"
#include "rs_bindings_from_cc/importers/function_template.h"
#include "rs_bindings_from_cc/importers/namespace.h"
#include "rs_bindings_from_cc/importers/typedef_name.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Mangle.h"
#include "clang/AST/RawCommentList.h"

namespace crubit {

// Iterates over the AST created from the invocation's entry headers and
// creates an intermediate representation of the import (`IR`) into the
// invocation object.
class Importer final : public ImportContext {
 public:
  explicit Importer(Invocation& invocation, clang::ASTContext& ctx,
                    clang::Sema& sema)
      : ImportContext(invocation, ctx, sema),
        mangler_(ABSL_DIE_IF_NULL(ctx_.createMangleContext())) {
    decl_importers_.push_back(
        std::make_unique<ClassTemplateDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<CXXRecordDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<EnumDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<FriendDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<FunctionDeclImporter>(*this));
    decl_importers_.push_back(
        std::make_unique<FunctionTemplateDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<NamespaceDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<TypedefNameDeclImporter>(*this));
  }

  // Import all visible declarations from a translation unit.
  void Import(clang::TranslationUnitDecl* decl);

 protected:
  // Implementation of `ImportContext`
  void ImportDeclsFromDeclContext(
      const clang::DeclContext* decl_context) override;
  IR::Item ImportUnsupportedItem(const clang::Decl* decl,
                                 std::string error) override;
  IR::Item ImportUnsupportedItem(const clang::Decl* decl,
                                 std::set<std::string> errors) override;
  std::optional<IR::Item> ImportDecl(clang::Decl* decl) override;
  std::optional<IR::Item> GetImportedItem(const clang::Decl* decl) override;
  std::vector<ItemId> GetItemIdsInSourceOrder(clang::Decl* decl) override;
  std::string GetMangledName(const clang::NamedDecl* named_decl) const override;
  BazelLabel GetOwningTarget(const clang::Decl* decl) const override;
  bool IsFromCurrentTarget(const clang::Decl* decl) const override;
  absl::StatusOr<UnqualifiedIdentifier> GetTranslatedName(
      const clang::NamedDecl* named_decl) const override;
  absl::StatusOr<Identifier> GetTranslatedIdentifier(
      const clang::NamedDecl* named_decl) const override {
    CRUBIT_ASSIGN_OR_RETURN(UnqualifiedIdentifier unqualified,
                            GetTranslatedName(named_decl));
    Identifier* identifier = std::get_if<Identifier>(&unqualified);
    CHECK(identifier) << "Incorrectly called with a special name";
    return *identifier;
  }
  std::optional<std::string> GetComment(const clang::Decl* decl) const override;
  std::string ConvertSourceLocation(clang::SourceLocation loc) const override;
  absl::StatusOr<MappedType> ConvertQualType(
      clang::QualType qual_type,
      std::optional<clang::tidy::lifetimes::ValueLifetimes>& lifetimes,
      std::optional<clang::RefQualifierKind> ref_qualifier_kind,
      bool nullable = true) override;

  void MarkAsSuccessfullyImported(const clang::TypeDecl* decl) override;
  bool HasBeenAlreadySuccessfullyImported(
      const clang::TypeDecl* decl) const override;
  bool EnsureSuccessfullyImported(clang::TypeDecl* decl) override {
    // First, return early so that we avoid re-entrant imports.
    if (HasBeenAlreadySuccessfullyImported(decl)) return true;
    (void)GetDeclItem(decl->getCanonicalDecl());
    return HasBeenAlreadySuccessfullyImported(decl);
  }

 private:
  class SourceOrderKey;
  class SourceLocationComparator;

  // Returns a SourceOrderKey for the given `decl` that should be used for
  // ordering Items.
  SourceOrderKey GetSourceOrderKey(const clang::Decl* decl) const;
  // Returns a SourceOrderKey for the given `comment` that should be used for
  // ordering Items.
  SourceOrderKey GetSourceOrderKey(const clang::RawComment* comment) const;

  // Returns a name for `decl` that should be used for ordering declarations.
  std::string GetNameForSourceOrder(const clang::Decl* decl) const;

  // Returns the item ids of template instantiations that have been triggered
  // from the current target.  The returned items are in an arbitrary,
  // deterministic/reproducible order.
  std::vector<ItemId> GetOrderedItemIdsOfTemplateInstantiations() const;

  std::optional<IR::Item> GetDeclItem(clang::Decl* decl) override;
  // Stores the comments of this target in source order.
  void ImportFreeComments();

  // Converts a type to a MappedType.
  //
  // ref_qualifier_kind is the member function reference qualifier, e.g., `&`
  // means Lvalue-reference-qualified, `&&` means Rvalue-reference-qualified.
  // This is only meaningful and hence populated when converting the type of
  // the `this` pointer.
  absl::StatusOr<MappedType> ConvertType(
      const clang::Type* type,
      std::optional<clang::tidy::lifetimes::ValueLifetimes>& lifetimes,
      std::optional<clang::RefQualifierKind> ref_qualifier_kind, bool nullable);
  absl::StatusOr<MappedType> ConvertTypeDecl(clang::TypeDecl* decl);

  // Converts `type` into a MappedType, after first importing the Record behind
  // the template instantiation.
  absl::StatusOr<MappedType> ConvertTemplateSpecializationType(
      const clang::TemplateSpecializationType* type);

  // The different decl importers. Note that order matters: the first importer
  // to successfully match a decl "wins", and no other importers are tried.
  std::vector<std::unique_ptr<DeclImporter>> decl_importers_;
  std::unique_ptr<clang::MangleContext> mangler_;
  absl::flat_hash_map<const clang::Decl*, std::optional<IR::Item>>
      import_cache_;
  absl::flat_hash_set<const clang::ClassTemplateSpecializationDecl*>
      class_template_instantiations_;
  std::vector<const clang::RawComment*> comments_;

  // Set of decls that have been successfully imported (i.e. that will be
  // present in the IR output / that will not produce dangling ItemIds in the IR
  // output).
  absl::flat_hash_set<const clang::TypeDecl*> known_type_decls_;
};  // class Importer

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_
