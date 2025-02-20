use crate::{
    fuel_prelude::fuel_tx::StorageSlot,
    ir_generation::{
        const_eval::compile_constant_expression_to_constant, storage::serialize_to_storage_slots,
    },
    language::ty,
    metadata::MetadataManager,
    Engines,
};
use sway_error::{
    error::CompileError,
    handler::{ErrorEmitted, Handler},
};
use sway_ir::{Context, Module};
use sway_types::state::StateIndex;

impl ty::TyStorageDecl {
    pub(crate) fn get_initialized_storage_slots(
        &self,
        handler: &Handler,
        engines: &Engines,
        context: &mut Context,
        md_mgr: &mut MetadataManager,
        module: Module,
    ) -> Result<Vec<StorageSlot>, ErrorEmitted> {
        handler.scope(|handler| {
            let storage_slots = self
                .fields
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    f.get_initialized_storage_slots(
                        engines,
                        context,
                        md_mgr,
                        module,
                        &StateIndex::new(i),
                    )
                })
                .filter_map(|s| s.map_err(|e| handler.emit_err(e)).ok())
                .flatten()
                .collect::<Vec<_>>();

            Ok(storage_slots)
        })
    }
}

impl ty::TyStorageField {
    pub(crate) fn get_initialized_storage_slots(
        &self,
        engines: &Engines,
        context: &mut Context,
        md_mgr: &mut MetadataManager,
        module: Module,
        ix: &StateIndex,
    ) -> Result<Vec<StorageSlot>, CompileError> {
        compile_constant_expression_to_constant(
            engines,
            context,
            md_mgr,
            module,
            None,
            None,
            &self.initializer,
        )
        .map(|constant| serialize_to_storage_slots(&constant, context, ix, &constant.ty, &[]))
    }
}
