use crate::wgpu;

/// A builder type to simplify the process of creating a render pass descriptor.
#[derive(Debug, Default)]
pub struct Builder<'a> {
    color_attachments: Vec<wgpu::RenderPassColorAttachmentDescriptor<'a>>,
    depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachmentDescriptor<'a>>,
}

/// A builder type to simplify the process of creating a render pass descriptor.
#[derive(Debug)]
pub struct ColorAttachmentDescriptorBuilder<'a> {
    descriptor: wgpu::RenderPassColorAttachmentDescriptor<'a>,
}

/// A builder type to simplify the process of creating a render pass descriptor.
#[derive(Debug)]
pub struct DepthStencilAttachmentDescriptorBuilder<'a> {
    descriptor: wgpu::RenderPassDepthStencilAttachmentDescriptor<'a>,
}

impl<'a> ColorAttachmentDescriptorBuilder<'a> {
    pub const DEFAULT_OPS: wgpu::Operations<wgpu::Color> = wgpu::Operations {
        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
        store: true,
    };

    /// Begin building a new render pass color attachment descriptor.
    fn new(attachment: &'a wgpu::TextureViewHandle) -> Self {
        ColorAttachmentDescriptorBuilder {
            descriptor: wgpu::RenderPassColorAttachmentDescriptor {
                attachment,
                resolve_target: None,
                ops: Self::DEFAULT_OPS,
            },
        }
    }

    /// Specify the resolve target for this render pass color attachment.
    pub fn resolve_target(mut self, target: Option<&'a wgpu::TextureView>) -> Self {
        self.descriptor.resolve_target = target.map(|t| &**t);
        self
    }

    /// Specify the resolve target for this render pass color attachment.
    pub fn resolve_target_handle(mut self, target: Option<&'a wgpu::TextureViewHandle>) -> Self {
        self.descriptor.resolve_target = target;
        self
    }

    /// Define operations for color pass
    pub fn ops(mut self, ops: wgpu::Operations<wgpu::Color>) -> Self {
        self.descriptor.ops = ops;
        self
    }
}

impl<'a> DepthStencilAttachmentDescriptorBuilder<'a> {
    pub const DEFAULT_DEPTH_OPS: wgpu::Operations<f32> = wgpu::Operations {
        load: wgpu::LoadOp::Clear(1.0),
        store: true,
    };
    pub const DEFAULT_STENCIL_OPS: wgpu::Operations<u32> = wgpu::Operations {
        load: wgpu::LoadOp::Clear(0),
        store: true,
    };

    fn new(attachment: &'a wgpu::TextureViewHandle) -> Self {
        DepthStencilAttachmentDescriptorBuilder {
            descriptor: wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment,
                depth_ops: Some(Self::DEFAULT_DEPTH_OPS),
                stencil_ops: Some(Self::DEFAULT_STENCIL_OPS),
            },
        }
    }

    /// Define operations for depth pass
    pub fn depth_ops(mut self, ops: wgpu::Operations<f32>) -> Self {
        self.descriptor.depth_ops = ops;
        self
    }

    /// Define operations for stencil pass
    pub fn stencil_ops(mut self, ops: wgpu::Operations<u32>) -> Self {
        self.descriptor.stencil_ops = ops;
        self
    }
}

impl<'a> Builder<'a> {
    pub const DEFAULT_COLOR_OPS: wgpu::Operations<wgpu::Color> =
        ColorAttachmentDescriptorBuilder::DEFAULT_OPS;
    pub const DEFAULT_DEPTH_OPS: wgpu::Operations<f32> =
        DepthStencilAttachmentDescriptorBuilder::DEFAULT_DEPTH_OPS;
    pub const DEFAULT_STENCIL_OPS: wgpu::Operations<u32> =
        DepthStencilAttachmentDescriptorBuilder::DEFAULT_STENCIL_OPS;

    /// Begin building a new render pass descriptor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a single color attachment descriptor to the render pass descriptor.
    ///
    /// Call this multiple times in succession to add multiple color attachments.
    pub fn color_attachment<F>(
        mut self,
        attachment: &'a wgpu::TextureViewHandle,
        color_builder: F,
    ) -> Self
    where
        F: FnOnce(ColorAttachmentDescriptorBuilder<'a>) -> ColorAttachmentDescriptorBuilder<'a>,
    {
        let builder = ColorAttachmentDescriptorBuilder::new(attachment);
        let descriptor = color_builder(builder).descriptor;
        self.color_attachments.push(descriptor);
        self
    }

    /// Add a depth stencil attachment to the render pass.
    ///
    /// This should only be called once, as only a single depth stencil attachment is valid. Only
    /// the attachment submitted last will be used.
    pub fn depth_stencil_attachment<F>(
        mut self,
        attachment: &'a wgpu::TextureViewHandle,
        depth_stencil_builder: F,
    ) -> Self
    where
        F: FnOnce(
            DepthStencilAttachmentDescriptorBuilder<'a>,
        ) -> DepthStencilAttachmentDescriptorBuilder<'a>,
    {
        let builder = DepthStencilAttachmentDescriptorBuilder::new(attachment);
        let descriptor = depth_stencil_builder(builder).descriptor;
        self.depth_stencil_attachment = Some(descriptor);
        self
    }

    /// Return the built color and depth attachments.
    pub fn into_inner(
        self,
    ) -> (
        Vec<wgpu::RenderPassColorAttachmentDescriptor<'a>>,
        Option<wgpu::RenderPassDepthStencilAttachmentDescriptor<'a>>,
    ) {
        let Builder {
            color_attachments,
            depth_stencil_attachment,
        } = self;
        (color_attachments, depth_stencil_attachment)
    }

    /// Begin a render pass with the specified parameters on the given encoder.
    pub fn begin(self, encoder: &'a mut wgpu::CommandEncoder) -> wgpu::RenderPass<'a> {
        let (color_attachments, depth_stencil_attachment) = self.into_inner();
        let descriptor = wgpu::RenderPassDescriptor {
            color_attachments: &color_attachments,
            depth_stencil_attachment,
        };
        encoder.begin_render_pass(&descriptor)
    }
}
