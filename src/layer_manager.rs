use std::{cell::RefCell, ops::Deref, rc::Rc};

use web_sys::HtmlCanvasElement;

use crate::virtual_context::VirtualContext;

pub struct RcLayerManager(pub Rc<RefCell<LayerManager>>);

impl From<LayerManager> for RcLayerManager {
    fn from(manager: LayerManager) -> Self {
        RcLayerManager(Rc::new(RefCell::new(manager)))
    }
}

impl Deref for RcLayerManager {
    type Target = Rc<RefCell<LayerManager>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for RcLayerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for RcLayerManager {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

pub struct LayerManager {
    layers: Vec<Layer>,
    next_id: usize,

    width: u32,
    height: u32,

    subscribers: Vec<Subscriber>,
    next_subscriber_id: usize,
}

impl LayerManager {
    pub fn new(width: u32, height: u32) -> Self {
        log::debug!("LayerManager new");
        Self {
            layers: vec![],
            next_id: 0,
            width,
            height,
            subscribers: vec![],
            next_subscriber_id: 0,
        }
    }

    pub fn push_layer(&mut self) {
        let id = self.next_id();
        self.layers.push(Layer::new(id, self.width, self.height));
    }

    pub fn get_layer(&self, id: usize) -> Option<&'_ Layer> {
        for layer in self.layers.iter() {
            if layer.id == id {
                return Some(layer);
            }
        }
        None
    }

    pub fn draw_in_context<F: Fn(&VirtualContext)>(&self, id: usize, f: F) {
        let layer = self.get_layer(id).unwrap();
        f(&layer.context);
        self.notify(Notification::Change { id });
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = &Layer> + '_ {
        self.layers.iter()
    }

    pub fn subscribe(&mut self, listener: Box<dyn Fn(Notification)>) -> usize {
        let id = self.next_subscriber_id();
        log::info!("subescribed. id = {}", id);
        self.subscribers.push(Subscriber::new(id, listener));
        id
    }

    fn notify(&self, notification: Notification) {
        for subscriber in self.subscribers.iter() {
            (subscriber.callback)(notification);
        }
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    fn next_subscriber_id(&mut self) -> usize {
        let id = self.next_subscriber_id;
        self.next_subscriber_id += 1;
        id
    }
}

pub struct Layer {
    id: usize,
    context: VirtualContext,
}

impl Layer {
    fn new(id: usize, width: u32, height: u32) -> Self {
        Self {
            id,
            context: VirtualContext::new_independent(width, height),
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_canvas(&self) -> &'_ HtmlCanvasElement {
        self.context.get_canvas()
    }
}

struct Subscriber {
    _id: usize,
    callback: Box<dyn Fn(Notification)>,
}

impl Subscriber {
    fn new(id: usize, callback: Box<dyn Fn(Notification)>) -> Self {
        Self { _id: id, callback }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Notification {
    Change { id: usize },
}
