import * as PIXI_TYPE from "pixi.js";
import { getWidth } from "../../CommonFunctions";
import * as Colours from "../Colours";

class RainDrop {
    /**
     * 
     * @param {PIXI_TYPE.Graphics} graphics 
     */
    constructor(graphics, destroy) {
        this.color = Colours.getRandomColour();
        this.ticksToExplode = 75
        this.lifetime = Math.random() * 220 + 30;
        this.graphics = graphics;
        this.x = Math.random() * getWidth();
        this.y = 0;
        this.height = 50;
        this.width = 10;
        this.next();
        this.destory = () => destroy(this);
    }

    next() {
        this.lifetime--;
        if (this.lifetime < 0) {
            if (this.lifetime < -this.ticksToExplode) {
                this.destory();
            } else {
                const stage = this.lifetime / -this.ticksToExplode;
                this.graphics.lineStyle(5, this.color, 1 - stage);
                this.graphics.drawCircle(this.x + this.width / 2, this.y + this.height, stage * 35);
            }
        } else {
            this.y += 3;
            this.graphics.lineStyle(1, this.color);
            this.graphics.beginFill(this.color);
            this.graphics.drawRoundedRect(this.x, this.y, this.width, this.height, 5);
            this.graphics.endFill();
        }
    }
}

export default RainDrop