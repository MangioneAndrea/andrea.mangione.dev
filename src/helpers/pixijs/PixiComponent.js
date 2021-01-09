
import * as PIXI from "pixi.js";
import "./rain/RainHandler";
/**
 * Prepare a Pixi application in a given element
 * @param {string | HTMLElement} component where to render the canvas
 * @param {*} options Any additional option to pass to the application
 * @returns {PIXI.Application}
 */
export const initPixi = (component, options = {}) => {
    if (typeof component === "string") {
        component = document.querySelector(component)
    }
    // Create a new application
    const app = new PIXI.Application({
        autoResize: true,
        resolution: devicePixelRatio,
        ...options
    });
    function resize() {
        app.renderer.resize(component.clientWidth, component.clientHeight);
    }
    // Listen for window resize events
    window.addEventListener('resize', resize);
    resize();

    component.appendChild(app.view);
    //app.renderer.backgroundColor = 0xFFFF00;
    return app;
};