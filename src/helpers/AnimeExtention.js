import anime from "animejs";
export const asyncAnime = (props) => {
  return new Promise((resolve) => {
    anime({
      ...props,
      complete: resolve,
    });
  });
};
