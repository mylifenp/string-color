# string-color

## Functions available
- `get_colour(identifier: String, format: String): String`: Returns a color string in the specified color format based on the identifier. Generates the same color for the same identifier.

## Color formats supported
- RGB: `"rgb"`
- HEX: `"hex"`
- HSL: `"hsl"`
- HWB: `"hwb"`
- CMYK: `"cmyk"`

## Small example
```JS
import { string_color } from "@mylifenp/string-color-wasm";

console.log(string_color("A","rgb"));
```
