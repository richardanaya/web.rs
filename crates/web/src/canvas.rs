use js::*;

pub struct CanvasContext(ExternRef);

impl CanvasContext {
    pub fn from_element(element: &ExternRef) -> Self {
        let get_context = js!(r#"
            function(element){
                return element.getContext("2d");
            }"#);
        let ctx_ref = get_context.invoke_and_return_object(&[element.into()]);
        CanvasContext(ctx_ref)
    }

    pub fn set_fill_style(&self, style: &str) {
        let set_fill_style = js!(r#"
            function(ctx, style){
                ctx.fillStyle = style;
            }"#);
        set_fill_style.invoke(&[(&self.0).into(), style.into()]);
    }

    pub fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        let fill_rect = js!(r#"
            function(ctx, x, y, width, height){
                ctx.fillRect(x,y,width,height);
            }"#);
        fill_rect.invoke(&[
            (&self.0).into(),
            x.into(),
            y.into(),
            width.into(),
            height.into(),
        ]);
    }

    pub fn clear_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        let clear_rect = js!(r#"
            function(ctx, x, y, width, height){
                ctx.clearRect(x,y,width,height);
            }"#);
        clear_rect.invoke(&[
            (&self.0).into(),
            x.into(),
            y.into(),
            width.into(),
            height.into(),
        ]);
    }

    pub fn set_font(&self, font: &str) {
        let set_font = js!(r#"
            function(ctx, font){
                ctx.font = font;
            }"#);
        set_font.invoke(&[(&self.0).into(), font.into()]);
    }

    pub fn set_text_align(&self, align: &str) {
        let set_text_align = js!(r#"
            function(ctx, align){
                ctx.textAlign = align;
            }"#);
        set_text_align.invoke(&[(&self.0).into(), align.into()]);
    }

    pub fn set_text_baseline(&self, baseline: &str) {
        let set_text_baseline = js!(r#"
            function(ctx, baseline){
                ctx.textBaseline = baseline;
            }"#);
        set_text_baseline.invoke(&[(&self.0).into(), baseline.into()]);
    }

    pub fn fill_text(&self, text: &str, x: f64, y: f64) {
        let fill_text = js!(r#"
            function(ctx, text, x, y){
                ctx.fillText(text,x,y);
            }"#);
        fill_text.invoke(&[(&self.0).into(), text.into(), x.into(), y.into()]);
    }

    pub fn measure_text(&self, text: &str) -> f64 {
        let measure_text = js!(r#"
            function(ctx, text){
                return ctx.measureText(text).width;
            }"#);
        measure_text.invoke(&[(&self.0).into(), text.into()])
    }

    pub fn set_line_width(&self, width: f64) {
        let set_line_width = js!(r#"
            function(ctx, width){
                ctx.lineWidth = width;
            }"#);
        set_line_width.invoke(&[(&self.0).into(), width.into()]);
    }

    pub fn set_stroke_style(&self, style: &str) {
        let set_stroke_style = js!(r#"
            function(ctx, style){
                ctx.strokeStyle = style;
            }"#);
        set_stroke_style.invoke(&[(&self.0).into(), style.into()]);
    }

    pub fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        let stroke_rect = js!(r#"
            function(ctx, x, y, width, height){
                ctx.strokeRect(x,y,width,height);
            }"#);
        stroke_rect.invoke(&[
            (&self.0).into(),
            x.into(),
            y.into(),
            width.into(),
            height.into(),
        ]);
    }

    pub fn begin_path(&self) {
        let begin_path = js!(r#"
            function(ctx){
                ctx.beginPath();
            }"#);
        begin_path.invoke(&[(&self.0).into()]);
    }

    pub fn move_to(&self, x: f64, y: f64) {
        let move_to = js!(r#"
            function(ctx, x, y){
                ctx.moveTo(x,y);
            }"#);
        move_to.invoke(&[(&self.0).into(), x.into(), y.into()]);
    }

    pub fn line_to(&self, x: f64, y: f64) {
        let line_to = js!(r#"
            function(ctx, x, y){
                ctx.lineTo(x,y);
            }"#);
        line_to.invoke(&[(&self.0).into(), x.into(), y.into()]);
    }

    pub fn stroke(&self) {
        let stroke = js!(r#"
            function(ctx){
                ctx.stroke();
            }"#);
        stroke.invoke(&[(&self.0).into()]);
    }

    pub fn close_path(&self) {
        let close_path = js!(r#"
            function(ctx){
                ctx.closePath();
            }"#);
        close_path.invoke(&[(&self.0).into()]);
    }

    pub fn fill(&self) {
        let fill = js!(r#"
            function(ctx){
                ctx.fill();
            }"#);
        fill.invoke(&[(&self.0).into()]);
    }

    pub fn arc(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        let arc = js!(r#"
            function(ctx, x, y, radius, start_angle, end_angle){
                ctx.arc(x,y,radius,start_angle,end_angle);
            }"#);
        arc.invoke(&[
            (&self.0).into(),
            x.into(),
            y.into(),
            radius.into(),
            start_angle.into(),
            end_angle.into(),
        ]);
    }

    pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
        let arc_to = js!(r#"
            function(ctx, x1, y1, x2, y2, radius){
                ctx.arcTo(x1,y1,x2,y2,radius);
            }"#);
        arc_to.invoke(&[
            (&self.0).into(),
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            radius.into(),
        ]);
    }

    pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        let bezier_curve_to = js!(r#"
            function(ctx, cp1x, cp1y, cp2x, cp2y, x, y){
                ctx.bezierCurveTo(cp1x,cp1y,cp2x,cp2y,x,y);
            }"#);
        bezier_curve_to.invoke(&[
            (&self.0).into(),
            cp1x.into(),
            cp1y.into(),
            cp2x.into(),
            cp2y.into(),
            x.into(),
            y.into(),
        ]);
    }

    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
        let quadratic_curve_to = js!(r#"
            function(ctx, cpx, cpy, x, y){
                ctx.quadraticCurveTo(cpx,cpy,x,y);
            }"#);
        quadratic_curve_to.invoke(&[(&self.0).into(), cpx.into(), cpy.into(), x.into(), y.into()]);
    }

    pub fn rect(&self, x: f64, y: f64, width: f64, height: f64) {
        let rect = js!(r#"
            function(ctx, x, y, width, height){
                ctx.rect(x,y,width,height);
            }"#);
        rect.invoke(&[
            (&self.0).into(),
            x.into(),
            y.into(),
            width.into(),
            height.into(),
        ]);
    }

    pub fn clip(&self) {
        let clip = js!(r#"
            function(ctx){
                ctx.clip();
            }"#);
        clip.invoke(&[(&self.0).into()]);
    }

    pub fn draw_image(&self, image: &ExternRef, dx: f64, dy: f64) {
        let draw_image = js!(r#"
            function(ctx, image, dx, dy){
                ctx.drawImage(image,dx,dy);
            }"#);
        draw_image.invoke(&[(&self.0).into(), image.into(), dx.into(), dy.into()]);
    }

    pub fn draw_image_with_size(
        &self,
        image: &ExternRef,
        dx: f64,
        dy: f64,
        dwidth: f64,
        dheight: f64,
    ) {
        let draw_image_with_size = js!(r#"
            function(ctx, image, dx, dy, dwidth, dheight){
                ctx.drawImage(image,dx,dy,dwidth,dheight);
            }"#);
        draw_image_with_size.invoke(&[
            (&self.0).into(),
            image.into(),
            dx.into(),
            dy.into(),
            dwidth.into(),
            dheight.into(),
        ]);
    }

    pub fn draw_image_with_source(
        &self,
        image: &ExternRef,
        sx: f64,
        sy: f64,
        swidth: f64,
        sheight: f64,
        dx: f64,
        dy: f64,
        dwidth: f64,
        dheight: f64,
    ) {
        let draw_image_with_source = js!(r#"
            function(ctx, image, sx, sy, swidth, sheight, dx, dy, dwidth, dheight){
                ctx.drawImage(image,sx,sy,swidth,sheight,dx,dy,dwidth,dheight);
            }"#);
        draw_image_with_source.invoke(&[
            (&self.0).into(),
            image.into(),
            sx.into(),
            sy.into(),
            swidth.into(),
            sheight.into(),
            dx.into(),
            dy.into(),
            dwidth.into(),
            dheight.into(),
        ]);
    }
}
