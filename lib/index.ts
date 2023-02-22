import { get_color } from "../pkg/string_color";

export const enum ColorFormat {
  RGB = "rgb",
  HEX = "hex",
  HSL = "hsl",
  HWB = "hwb",
  CMYK = "cmyk",
}

export const string_color = (
  str: string,
  format: ColorFormat = ColorFormat.HEX
) => {
  try {
    return get_color(str, format);
  } catch (err) {
    console.log("error", err);
  }
};
