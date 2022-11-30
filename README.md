# bundle_bundle

Extension trait for Bundle with one method `bundle` that provides an alternative to tuples for building anonymous bundles.

Supports Bevy 0.9

# Usage

Add to your project with:

```
cargo add bundle_bundle
```

Now where you used a tuple struct:

```rust
commands.spawn((
    Text2dBundle::from_section("message", text_style),
    Background(Color::NAVY),
    MessageMarkerComponent,
));
```

You can instead use:

```rust
use bundle_bundle::BundleBundleExt;

commands.spawn(
    Text2dBundle::from_section("message", text_style)
    .bundle(BackgroundColor(Color::NAVY))
    .bundle(MessageMarkerComponent)
);
```

# Examples
```
cargo run --example hello_world
```