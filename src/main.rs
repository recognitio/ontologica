#![allow(dead_code)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate petgraph;
extern crate rand;

use pest::Parser;

mod ontology;

use ontology::Ontology;

use std::collections::hash_map::HashMap;

#[derive(Parser)]
#[grammar = "ontologica.pest"]
pub struct OntologicaParser;

pub fn main()
{
	//println!("recompile");
	let text = include_str!("test.onl");

	let parsed_text = OntologicaParser::parse(Rule::ontology, &text)
		.expect("The ontology could not be parsed").next().unwrap();

	let mut ontology = Ontology::new();

	//let mut declarations = parsed_text.into_inner();

	for declaration in parsed_text.into_inner()
	{
		let mut tokens = declaration.into_inner().peekable();

		let tag_set = tokens.next().unwrap();

		let tags =
		{
			let mut tags = Vec::new();
			for tag in tag_set.into_inner() {tags.push(tag.as_str()); ontology.add_vertex(tag.as_str());}

			tags
		};

		if !tokens.peek().is_none()
		{
			if tokens.peek().unwrap().as_rule() == Rule::specialization
			{
				let mut specialization = tokens.next().unwrap().into_inner();

				let superset_tag = specialization.next().unwrap().as_str();
				ontology.add_vertex(superset_tag);

				//TODO: handle case where there is no spec_list - done!

				if let Some(spec_list) = specialization.next()
				{
					let specs = 
					{
						let mut specs = HashMap::new();

						for spec in spec_list.into_inner()
						{
							//TODO: loudly handle case where the values of spec_list do not strictly form a set.
							let mut spec_tokens = spec.into_inner();

							let spec_param = spec_tokens.next().unwrap().as_str().to_owned();

							let mut vals = Vec::new();

							for t in spec_tokens
							{
								vals.push(t.as_str().to_owned());
							}

							specs.insert(spec_param, vals);
						}
						
						specs
					};

					println!("{:?} : {} | {:?}", tags, superset_tag, specs); //So, I'm guessing here we want to add the spec list to the arc, NOT the vector? One hashmap per arc.
					for tag in tags {ontology.add_arc(tag, superset_tag, ontology::Arrow::IsSpecializedSubsetOf(specs.clone()));} //TODO we don't actually need this clone? or do we?
				}
				else
				{
					println!("{:?} : {}", tags, superset_tag); //no specialization list - so just a normal supernymous arc.
					for tag in tags {ontology.add_arc(tag, superset_tag, ontology::Arrow::IsSubsetOf);} //TODO we don't actually need this clone? or do we?
				}
			}

			if !tokens.peek().is_none() && tokens.peek().unwrap().as_rule() == Rule::composition
			{

			}
		}
		
			/*
			Rule::factorization =>
			{
				let mut tokens = instruction.into_inner();

				let set = tokens.next().unwrap().as_str();

				ontology
					.add_vertex(set);

				while let Some(token) = tokens.next()
				{
					let factor = token.as_str();

					ontology
						.add_vertex(factor)
						.add_arc(set, factor, Arrow::Factor);
				}
			},
			Rule::supernymy =>
			{
				let mut tokens = instruction.into_inner();

				let set = tokens.next().unwrap().as_str();

				ontology.add_vertex(set);

				while let Some(token) = tokens.next()
				{
					let term = token.as_str();

					ontology
						.add_vertex(term)
						.add_arc(set, term, Arrow::Term);
				}
			},
			Rule::link =>
			{
				let mut tokens = instruction.into_inner();
				let first = tokens.next().unwrap().as_str();
				let first_property = tokens.next().unwrap().as_str();
				let second = tokens.next().unwrap().as_str();
				let second_property = tokens.next().unwrap().as_str();

				ontology
					.add_vertex(first)
					.add_vertex(second)
					.add_arc(first, second, Arrow::Link(first_property.to_owned(), second_property.to_owned())); //symmetric, TODO: they take terms. They should actually take properties of sets. 
			}
			_ => unimplemented!()
			*/
	}

	println!("{:?}", ontology);

	/*let key = "optical-effect";//ontology.get_random_concept_key();
	let vex = ontology.get_outgoing_relations(key).unwrap();

	for v in vex
	{
		let (arr, nex) = v;

		println!("{:?} >-- {:?} --> {:?}", key, arr, nex);
	}

	let vex = ontology.get_outgoing_relations(key).unwrap();

	for v in vex
	{
		let (arr, nex) = v;

		println!("{:?} >-- {:?} --> {:?}", key, arr, nex);
	}*/
}
