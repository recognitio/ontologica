use std::{collections::hash_map::HashMap, vec::Vec};
use std::cell::RefCell;

// For $ in Arrow, the sentence Tail $ Head can be constructed.
#[derive(Debug, Clone)]
pub enum Arrow
{
	IsSubsetOf,
	IsSpecializedSubsetOf(HashMap<String, Vec<String>>),
	Meronym,
}
#[derive(Debug, Clone)]
pub struct Arc<Data>
{
	data: Data,
	source: String,
	target: String
}

impl<Data> Arc<Data>
{
	pub fn new(data: Data, source: String, target: String) -> Arc<Data>
	{
		Arc
		{
			data: data,
			source: source,
			target: target,
		}
	}

	pub fn is_loop(&self) -> bool
	{
		self.source == self.target
	}
}
#[derive(Debug, Clone)]
pub struct Vertex
{
	sources: Vec<usize>,
	targets: Vec<usize>,
}


impl Vertex
{
	pub fn new() -> Vertex
	{
		Vertex
		{
			sources: Vec::new(),
			targets: Vec::new(),
		}
	}
}
#[derive(Debug, Clone)]
pub struct Ontology
{
	vertices: HashMap<String, RefCell<Vertex>>,
	arcs: Vec<Arc<Arrow>>
}

impl Ontology // a dimultigraph
{
	pub fn new() -> Ontology
	{
		Ontology
		{
			vertices: HashMap::new(),
			arcs: Vec::new(),
		}
	}
	pub fn add_vertex(&mut self, label: &str) -> &mut Ontology
	{
		if !self.vertices.contains_key(label) {self.vertices.insert(label.to_owned(), RefCell::new(Vertex::new()));}

		self
	}

	pub fn add_arc(&mut self, source: &str, target: &str, data: Arrow) -> &mut Ontology
	{
		if let Some(source_concept_cell) = self.vertices.get(source)
		{
			if let Some(target_concept_cell) = self.vertices.get(target)
			{
				source_concept_cell.borrow_mut().targets.push(self.arcs.len());
				target_concept_cell.borrow_mut().sources.push(self.arcs.len());
				self.arcs.push(Arc::new(data, source.to_owned(), target.to_owned()));
			}
		}
		
		self
	}

	pub fn get_random_concept_key(&self) -> &str
	{
		use rand::{thread_rng, Rng};
		let mut rng = thread_rng();

		self.vertices.keys().nth(rng.gen_range(0, self.vertices.len())).unwrap().as_str()
	}

	pub fn get_outgoing_relations(&self, source: &str) -> Option<Vec<(Arrow, String)>>
	{
		if let Some(concept) = self.vertices.get(source)
		{
			let mut arcs = Vec::new();

			for &relation_index in concept.borrow().targets.as_slice()
			{
				let relation = self.arcs.get(relation_index).unwrap();
				arcs.push((relation.data.clone(), relation.target.clone()));
			}

			return Some(arcs);
		}

		None
	}
}