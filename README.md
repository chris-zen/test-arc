This is the paint code:

```rust
struct CustomWidget;

impl Widget<String> for CustomWidget {
  // ...
  fn paint(&mut self, ctx: &mut PaintCtx, _data: &String, _env: &Env) {
        let size = ctx.size();
        let half_size = size * 0.5;
        let center = Point::new(half_size.width, half_size.height);

        let arc = Arc {
            center,
            radii: Vec2::new(100.0, 100.0),
            start_angle: FRAC_PI_2,
            sweep_angle: FRAC_PI_2,
            x_rotation: 0.0,
        };

        let path = BezPath::from_vec(arc.append_iter(1e-9).collect());
        let color = Color::WHITE;
        ctx.stroke(path, &color, 6.0);
    }
}
```

This is what I see:

![screenshot](test-arc.png)