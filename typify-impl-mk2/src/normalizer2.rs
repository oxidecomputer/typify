use log::debug;

use crate::{
    bundler::Bundle,
    normalizer::{NormalizedSchemaGraph, SchemaletGraph},
    schemalet::{
        SchemaRef, Schemalet, SchemaletDetails, SchemaletMetadata, SchemaletValue,
        SchemaletValueArray, SchemaletValueInteger, SchemaletValueNumber, SchemaletValueObject,
        SchemaletValueString,
    },
    typify::Result,
};

#[derive(Debug, Default)]
pub struct Normalizer2 {
    graph: SchemaletGraph,
}

impl Normalizer2 {
    pub(crate) fn add(&mut self, bundle: &Bundle, id: impl AsRef<str>) -> Result<SchemaRef> {
        let id = id.as_ref();

        // Add the schemalets from the bundle...
        let root_ref = self.graph.add_nodes(bundle, id)?;

        Ok(root_ref)
    }

    pub(crate) fn to_normalized_graph(mut self) -> NormalizedSchemaGraph {
        let mut pass = 0;
        loop {
            pass += 1;
            debug!("simplifying... pass {pass}");

            // TODO 6/16/2026
            // As with the first version, we're just going to look at every
            // node every time. I need to think through termination conditions.

            let mut wip = self.graph.nodes.keys().cloned().collect::<Vec<_>>();

            for schema_ref in wip.drain(..) {
                let schemalet = self.graph.get(&schema_ref);

                let result = self.simplify2(&schema_ref, schemalet);
                match result {
                    Simplify2State::Stuck => {}
                    Simplify2State::Simplified { node, referenced } => {
                        self.graph.nodes.insert(schema_ref.clone(), node);
                        self.graph.nodes.extend(referenced);
                    }
                }
            }
        }
        todo!()
    }

    fn simplify2(&self, schema_ref: &SchemaRef, schemalet: &Schemalet) -> Simplify2State {
        let metadata = &schemalet.metadata;
        match &schemalet.details {
            SchemaletDetails::Anything => self.simplify2_anything(schema_ref, metadata),
            SchemaletDetails::Nothing => self.simplify2_nothing(schema_ref, metadata),
            SchemaletDetails::OneOf(subs) => self.simplify2_one_of(schema_ref, metadata, subs),
            SchemaletDetails::AnyOf(subs) => self.simplify2_any_of(schema_ref, metadata, subs),
            SchemaletDetails::AllOf(subs) => self.simplify2_all_of(schema_ref, metadata, subs),
            SchemaletDetails::Not(sub) => self.simplify2_not(schema_ref, metadata, sub),
            SchemaletDetails::IfThen(if_sub, then_sub) => {
                self.simplify2_if_then(schema_ref, metadata, if_sub, then_sub)
            }
            SchemaletDetails::IfThenElse(if_sub, then_sub, else_sub) => {
                self.simplify2_if_then_else(schema_ref, metadata, if_sub, then_sub, else_sub)
            }
            SchemaletDetails::RawRef(target) => self.simplify2_raw_ref(schema_ref, metadata, target),
            SchemaletDetails::RawDynamicRef(target) => {
                self.simplify2_raw_dynamic_ref(schema_ref, metadata, target)
            }
            SchemaletDetails::Constant(value) => {
                self.simplify2_constant(schema_ref, metadata, value)
            }
            SchemaletDetails::Value(schemalet_value) => {
                self.simplify2_value(schema_ref, metadata, schemalet_value)
            }
            SchemaletDetails::ExclusiveOneOf(subs) => {
                self.simplify2_exclusive_one_of(schema_ref, metadata, subs)
            }
            SchemaletDetails::ResolvedRef(target) => {
                self.simplify2_resolved_ref(schema_ref, metadata, target)
            }
            SchemaletDetails::ResolvedDynamicRef(target) => {
                self.simplify2_resolved_dynamic_ref(schema_ref, metadata, target)
            }
            SchemaletDetails::YesNo { yes, no } => {
                self.simplify2_yes_no(schema_ref, metadata, yes, no)
            }
            SchemaletDetails::StringOf(sub) => self.simplify2_string_of(schema_ref, metadata, sub),
        }
    }

    fn simplify2_anything(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_nothing(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_one_of(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        subs: &[SchemaRef],
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_any_of(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        subs: &[SchemaRef],
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_all_of(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        subs: &[SchemaRef],
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_not(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        sub: &SchemaRef,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_if_then(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        if_sub: &SchemaRef,
        then_sub: &SchemaRef,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_if_then_else(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        if_sub: &SchemaRef,
        then_sub: &SchemaRef,
        else_sub: &SchemaRef,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_raw_ref(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        target: &str,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_raw_dynamic_ref(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        target: &str,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_constant(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        value: &serde_json::Value,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_exclusive_one_of(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        subs: &[SchemaRef],
    ) -> Simplify2State {
        // TODO 6/16/2026
        // We already know that each subschema is independent. The only
        // thing we could conceivably simplify is if there are
        // "Nothing" schemas embedded in there. Or if we're able to
        // winnow it down to a single case.
        //
        // We'll know that there is no possible additional
        // simplification that could be performed on this node once all
        // its children report that they affirmatively cannot Nothing
        // (i.e. unsatisfiable).

        Simplify2State::Stuck
    }

    fn simplify2_resolved_ref(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        target: &SchemaRef,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_resolved_dynamic_ref(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        target: &SchemaRef,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_yes_no(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        yes: &SchemaRef,
        no: &[SchemaRef],
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_string_of(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        sub: &SchemaRef,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_value(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        schemalet_value: &SchemaletValue,
    ) -> Simplify2State {
        match schemalet_value {
            SchemaletValue::Boolean => self.simplify2_boolean(schema_ref, metadata),
            SchemaletValue::Array(array) => self.simplify2_array(schema_ref, metadata, array),
            SchemaletValue::Object(object) => self.simplify2_object(schema_ref, metadata, object),
            SchemaletValue::String(string) => self.simplify2_string(schema_ref, metadata, string),
            SchemaletValue::Integer(integer) => {
                self.simplify2_integer(schema_ref, metadata, integer)
            }
            SchemaletValue::Number(number) => self.simplify2_number(schema_ref, metadata, number),
            SchemaletValue::Null => self.simplify2_null(schema_ref, metadata),
        }
    }

    fn simplify2_boolean(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_array(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        array: &SchemaletValueArray,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_object(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        object: &SchemaletValueObject,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_string(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        string: &SchemaletValueString,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_integer(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        integer: &SchemaletValueInteger,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_number(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
        number: &SchemaletValueNumber,
    ) -> Simplify2State {
        todo!()
    }

    fn simplify2_null(
        &self,
        schema_ref: &SchemaRef,
        metadata: &SchemaletMetadata,
    ) -> Simplify2State {
        todo!()
    }
}

enum Simplify2State {
    Stuck,
    Simplified {
        node: Schemalet,
        referenced: Vec<(SchemaRef, Schemalet)>,
    },
}
