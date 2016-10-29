== Scene Metalanguage ==

Inspired by POV-Ray.

=== Specifying intersectables ===

```
sphere {
  <centre>, radius;
  modifiers...
};
```

Will be added to scene when specified, unless they are saved with `let`.

```
let myobject = sphere { ... };
```

Saved objects are recalled using the keyword `object`.

```
object {
  myobject;
  modifiers...
}
```

=== Specifying materials ===

The modifiers are texture, interior, and transformation.

```
sphere {
  centre, radius;
  texture {
    lambertian {
      colour {
        red
      }
    }
  }
};
```

You can always flatten `thing { subthing { ... } }` into just `thing subthing ...`,
so the above can be written

```
sphere {
  centre, radius;
  texture lambertian colour red
}
```
