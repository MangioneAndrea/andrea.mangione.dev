import anime from "animejs";
import { oneRandomOutOf } from "./CommonFunctions";

const easeTypes = [
  "Sine",
  "Cubic",
  "Quint",
  "Circ",
  "Elastic",
  "Bounce",
  "Back",
  "Expo",
  "Quart",
  "Quad",
];

const easeIn = easeTypes.map((type) => `easeIn${type}`);
const easeOut = easeTypes.map((type) => `easeOut${type}`);
const easeInOut = easeTypes.map((type) => `easeInOut${type}`);
export const asyncAnime = (props) => {
  return new Promise((resolve) => {
    anime({
      ...props,
      complete: resolve,
    });
  });
};

/**
 * 
 * @param {"in" | "out" | "inOut" | "all"} which 
 */
export const oneRandomEase = (which="all") => {
  switch (which) {
    case "in":
      return oneRandomOutOf(easeIn);
    case "out":
      return oneRandomOutOf(easeOut);
    case "inOut":
      return oneRandomOutOf(easeInOut);
      case "all"
    default:
      return oneRandomOutOf([...easeInOut, ...easeIn, ...easeOut]);
  }
};
