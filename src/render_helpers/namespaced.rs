use smithay::backend::renderer::element::{Element, Id, UnderlyingStorage};
use smithay::backend::renderer::utils::CommitCounter;
use smithay::backend::renderer::Renderer;
use smithay::backend::renderer::element::RenderElement;
use smithay::utils::{Physical, Rectangle, Scale, Transform};
use smithay::backend::allocator::Buffer;

#[derive(Debug)]
pub struct NamespacedRenderElement<E> {
    id: Id,
    inner: E,
}

impl<E: Element> NamespacedRenderElement<E> {
    pub fn new(inner: E, ns: usize) -> Self {
        Self {
            id: inner.id().clone().namespaced(ns),
            inner,
        }
    }
}

impl<E: Element> Element for NamespacedRenderElement<E> {
    fn id(&self) -> &Id {
        &self.id
    }

    fn current_commit(&self) -> CommitCounter {
        self.inner.current_commit()
    }

    fn src(&self) -> Rectangle<f64, Buffer> {
        self.inner.src()
    }

    fn geometry(&self, scale: Scale<f64>) -> Rectangle<i32, Physical> {
        self.inner.geometry(scale)
    }

    fn transform(&self) -> Transform {
        self.inner.transform()
    }

    fn damage_since(
        &self,
        scale: Scale<f64>,
        commit: Option<CommitCounter>,
    ) -> Vec<Rectangle<i32, Physical>> {
        self.inner.damage_since(scale, commit)
    }

    fn opaque_regions(&self, scale: Scale<f64>) -> Vec<Rectangle<i32, Physical>> {
        self.inner.opaque_regions(scale)
    }

    fn alpha(&self) -> f32 {
        self.inner.alpha()
    }

    fn is_opaque(&self) -> bool {
        self.inner.is_opaque()
    }
}

impl<E, R> RenderElement<R> for NamespacedRenderElement<E>
where
    E: RenderElement<R>,
    R: smithay::backend::renderer::Renderer,
{
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: Rectangle<f64, Buffer>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        opaque_regions: &[Rectangle<i32, Physical>],
        cache: Option<&smithay::utils::user_data::UserDataMap>,
    ) -> Result<(), R::Error> {
        RenderElement::<R>::draw(&self.inner, frame, src, dst, damage, opaque_regions, cache)
    }

    fn underlying_storage(&self, renderer: &mut R) -> Option<UnderlyingStorage<'_>> {
        self.inner.underlying_storage(renderer)
    }
}
