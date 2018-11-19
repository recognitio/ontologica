use std::collections::HashMap;

use indexmap::IndexMap;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(EnumKind)]
#[enum_kind(AtomKind)]
pub enum Atom
{
	IsHyponymOf,
	IsSpecializedSubsetOf(HashMap<String, Vec<String>>),
	Meronym,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Vertex
{
	ingress_indices: Vec<usize>,
	egress_indices: Vec<usize>,
	regress_indices: Vec<usize>,
}

impl Vertex
{
	fn new() -> Vertex
	{
		Vertex
		{
			ingress_indices: Vec::new(),
			egress_indices: Vec::new(),
			regress_indices: Vec::new(),
		}
	}

	pub fn ingress_indices(&self) -> Vec<usize>
	{
		self.ingress_indices.clone()
	}

	pub fn egress_indices(&self) -> Vec<usize>
	{
		self.egress_indices.clone()
	}

	pub fn regress_indices(&self) -> Vec<usize>
	{
		self.regress_indices.clone()
	}

	pub fn weak_egress_indices(&self) -> Vec<usize>
	{
		self.egress_indices.iter().chain(self.regress_indices.iter()).map(|i| i.clone()).collect()
	}

	pub fn weak_ingress_indices(&self) -> Vec<usize>
	{
		self.ingress_indices.iter().chain(self.regress_indices.iter()).map(|i| i.clone()).collect()
	}

	pub fn transgress_indices(&self) -> Vec<usize>
	{
		self.ingress_indices.iter().chain(self.egress_indices.iter()).map(|i| i.clone()).collect()
	}

	pub fn arrow_indices(&self) -> Vec<usize>
	{
		self.ingress_indices.iter().chain(self.egress_indices.iter()).chain(self.regress_indices.iter()).map(|i| i.clone()).collect()
	}
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Arrow
{
	data: Atom,
	source_index: String,
	target_index: String,
}

impl Arrow
{
	fn new(data: Atom, source_index: String, target_index: String) -> Arrow
	{
		Arrow
		{
			data: data,
			source_index: source_index,
			target_index: target_index,
		}
	}

	pub fn data(&self) -> &Atom
	{
		&self.data
	}

	pub fn source_index(&self) -> &str
	{
		self.source_index.as_str()
	}

	pub fn target_index(&self) -> &str
	{
		self.target_index.as_str()
	}
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Ontology
{
	vertices: IndexMap<String, Vertex>,
	arrows: Vec<Arrow>,
}

impl Ontology
{
	pub fn new() -> Ontology
	{
		Ontology
		{
			vertices: IndexMap::new(),
			arrows: Vec::new(),
		}
	}

	pub fn choose_vertex_name(&self) -> String
	{
		self.vertices.keys().next().unwrap_or(&"None :(".to_string()).to_string()
	}

	pub fn vertex(&self, label: String) -> Vertex
	{
		self.vertices.get(&label).unwrap_or(&Vertex::new()).clone()
	}

	pub fn add_vertex(&mut self, vertex_label: String)
	{
		if !self.vertices.contains_key(&vertex_label) {self.vertices.insert(vertex_label, Vertex::new());};
	}

	pub fn add_arrow(&mut self, arrow_data: Atom, source_label: &String, target_label: &String)
	{
		if self.vertices.get(source_label).is_none() || self.vertices.get(target_label).is_none()
		{
			panic!("declared arrow has nonexistent source and/or target");
		}
		else
		{
			let arrow_index = self.arrows.len();
			self.arrows.push(Arrow::new(arrow_data, source_label.clone(), target_label.clone()));

			if source_label == target_label
			{
				self.vertices.get_mut(source_label).unwrap().regress_indices.push(arrow_index);
			}
			else
			{
				self.vertices.get_mut(source_label).unwrap().egress_indices.push(arrow_index);
				self.vertices.get_mut(target_label).unwrap().ingress_indices.push(arrow_index);
			}
		}
	}

	pub fn arrow_data(&self, arrow_index: usize) -> Option<&Atom>
	{
		Some(self.arrows.get(arrow_index)?.data())
	}

	pub fn ingresses(&self, vertex_label: &String) -> Vec<Arrow>
	{
		self.vertices[vertex_label].ingress_indices().iter().map(|i| self.arrows[i.clone()].clone()).collect()
	}
	
	pub fn egresses(&self, vertex_label: &String) -> Vec<Arrow>
	{
		self.vertices[vertex_label].egress_indices().iter().map(|i| self.arrows[i.clone()].clone()).collect()
	}

	pub fn regresses(&self, vertex_label: &String) -> Vec<Arrow>
	{
		self.vertices[vertex_label].regress_indices().iter().map(|i| self.arrows[i.clone()].clone()).collect()
	}

	pub fn weak_ingresses(&self, vertex_label: &String) -> Vec<Arrow>
	{
		self.vertices[vertex_label].weak_ingress_indices().iter().map(|i| self.arrows[i.clone()].clone()).collect()
	}

	pub fn weak_egresses(&self, vertex_label: &String) -> Vec<Arrow>
	{
		self.vertices[vertex_label].weak_egress_indices().iter().map(|i| self.arrows[i.clone()].clone()).collect()
	}

	pub fn transgresses(&self, vertex_label: &String) -> Vec<Arrow>
	{
		self.vertices[vertex_label].transgress_indices().iter().map(|i| self.arrows[i.clone()].clone()).collect()
	}

	pub fn arrows(&self, vertex_label: &String) -> Vec<Arrow>
	{
		self.vertices[vertex_label].arrow_indices().iter().map(|i| self.arrows[i.clone()].clone()).collect()
	}

	pub fn hypernyms(&self, vertex_label: &String) -> Vec<String> //TODO - be certain that hypernyms may only be egresses.
	{
		self.egresses(vertex_label).iter().filter(|a| AtomKind::from(&a.data) == AtomKind::IsHyponymOf || AtomKind::from(&a.data) == AtomKind::IsSpecializedSubsetOf).map(|a| a.target_index().to_string()).collect()
	}

	pub fn hyponyms(&self, vertex_label: &String) -> Vec<String>
	{
		self.ingresses(vertex_label).iter().filter(|a| AtomKind::from(&a.data) == AtomKind::IsHyponymOf || AtomKind::from(&a.data) == AtomKind::IsSpecializedSubsetOf).map(|a| a.source_index().to_string()).collect()
	}

	pub fn vertex_labels(&self) -> Vec<String>
	{
		self.vertices.keys().map(|k| k.clone()).collect()
	}
}