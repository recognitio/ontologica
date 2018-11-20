#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate indexmap;
#[macro_use]
extern crate enum_kinds;

mod ontology;

use pest::Parser;

pub use ontology::{Ontology, Atom};

#[derive(Parser)]
#[grammar = "ontologica.pest"]
pub struct OntologicaParser;

pub fn parse_ontology_source(text: &str) -> Ontology
{
	let declarations = OntologicaParser::parse(Rule::ontology, &text)
		.expect("the ontology could not be parsed").next().unwrap();

	let mut ontology = Ontology::new();

	for declaration in declarations.into_inner()
	{
		let mut concepts = Vec::new();
		let mut hypernyms = Vec::new();
		let mut holonyms = Vec::new();
		let mut types = Vec::new();

		for pair in declaration.into_inner()
		{
			let token = pair.clone();println!("{:?}", &token.as_str());

			match pair.as_rule()
			{
				Rule::tag =>
				{
					concepts.push(token.as_str().to_string());
				},
				Rule::aspect =>
				{
					for pair in token.into_inner()
					{
						let token = pair.clone();

						match pair.as_rule()
						{
							Rule::subsumption =>
							{
								for tag in token.into_inner()
								{
									hypernyms.push(tag.as_str().to_string())
								}
							},
							Rule::composition =>
							{
								for tag in token.into_inner()
								{
									holonyms.push(tag.as_str().to_string())
								}
							},
							Rule::instantiation =>
							{
								for tag in token.into_inner()
								{
									types.push(tag.as_str().to_string())
								}
							},
							_ => {}
						}
					}
				},
				_ => {}
			}
		}
		// Finished parsing declaration - now load into Ontology

		for concept in &concepts
		{
			ontology.add_vertex(concept.clone());
		}

		for hypernym in &hypernyms
		{
			ontology.add_vertex(hypernym.clone());
			
			for concept in &concepts
			{
				ontology.add_arrow(Atom::IsHyponymOf, &concept, &hypernym)
			}
		}

		for holonym in &holonyms
		{
			ontology.add_vertex(holonym.clone());
			
			for concept in &concepts
			{
				ontology.add_arrow(Atom::IsMeronymOf, &concept, &holonym)
			}
		}

		for set in &types
		{
			ontology.add_vertex(set.clone());
			
			for concept in &concepts
			{
				ontology.add_arrow(Atom::IsTokenOf, &concept, &set)
			}
		}
	}

	ontology
}

#[cfg(test)]
mod tests
{
	#[test]
	fn it_works()
	{
		let s = include_str!("bio.onl");
		let o = ::parse_ontology_source(s);
		println!("{:?}", &o);
	}
}