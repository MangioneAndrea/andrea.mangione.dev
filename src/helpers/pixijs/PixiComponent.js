
import * as PIXI from "pixi.js";
import "./rain/RainHandler";

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