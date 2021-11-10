use druid::piet::ImageFormat;
use druid::widget::{prelude::*, Button, Flex, Image, SizedBox};
use druid::{
    AppLauncher, Color, Data, Env, ImageBuf, Lens, UnitPoint, Widget, WidgetExt, WindowDesc,
};
use tiny_renderer::lessons::{lessons, Lesson};
use tiny_renderer::rgb_image::RGBImage;

#[derive(Clone, Data, Lens)]
struct AppState {
    selected_lesson: LessonState,
}

#[derive(Clone)]
struct LessonState {
    lesson: Lesson,
}

impl LessonState {
    fn new(lesson: Lesson) -> LessonState {
        LessonState { lesson }
    }
}

impl Data for LessonState {
    fn same(&self, other: &Self) -> bool {
        self.lesson.name == other.lesson.name
    }
}

pub fn main() {
    let main_window = WindowDesc::new(build_root_widget())
        .title("TinyRenderer")
        .window_size((640.0, 480.0));

    let state = AppState {
        selected_lesson: LessonState::new(lessons()[0]),
    };
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    let mut column = Flex::column();
    for lesson in lessons() {
        let button = Button::new(lesson.name)
            .on_click(move |_ctx, data: &mut AppState, _env| {
                data.selected_lesson = LessonState::new(lesson);
            })
            .padding(10.0);
        column.add_child(button)
    }

    Flex::row()
        .with_child(column)
        .with_default_spacer()
        .with_child(ReBuilder::new().lens(AppState::selected_lesson))
        .align_horizontal(UnitPoint::TOP_LEFT)
}

fn image_to_buffer(image: RGBImage) -> ImageBuf {
    let pixels: Vec<u8> = image
        .pixels
        .iter()
        .flat_map(|pixel| vec![pixel.r, pixel.g, pixel.b])
        .collect();
    return ImageBuf::from_raw(
        pixels,
        ImageFormat::Rgb,
        image.width as usize,
        image.height as usize,
    );
}

struct ReBuilder {
    inner: Box<dyn Widget<LessonState>>,
}

impl ReBuilder {
    fn new() -> ReBuilder {
        ReBuilder {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &LessonState) {
        self.inner = build_widget(data);
    }
}

impl Widget<LessonState> for ReBuilder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut LessonState, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &LessonState,
        env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &LessonState,
        data: &LessonState,
        _env: &Env,
    ) {
        if !old_data.same(data) {
            self.rebuild_inner(data);
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &LessonState,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &LessonState, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}

fn build_widget(state: &LessonState) -> Box<dyn Widget<LessonState>> {
    let image_data = image_to_buffer((state.lesson.renderer)());
    let image = Image::new(image_data);
    let sized = SizedBox::new(image);
    sized.border(Color::grey(0.6), 2.0).center().boxed()
}
