<script>
  import CodeEditor from "../../../../elements/codeEditor/CodeEditor.svelte";
  import LanguageList from "../../../../elements/containers/LanguageList.svelte";
  import Kotlin from "../../../../images/Logos/Kotlin.svg";
  import Arduino from "devicon/icons/arduino/arduino-original.svg";
  import ArduinoSketch from "./ArduinoSketch.svelte";
  import FaArrowRight from "svelte-icons/fa/FaArrowRight.svelte";
  import FaArrowDown from "svelte-icons/fa/FaArrowDown.svelte";
</script>

<div class="root">
  <a href="https://github.com/MangioneAndrea/kotlin-firmata" target="_blank">
    Kotlin firmata
  </a>
  <LanguageList icons={[Kotlin, Arduino]} />
  <p>
    This project helps you create arduino projects without c knowledge allowing
    multiplatforming. As kotlin is the main language for android native apps, it
    also helps you develop prototypes guided by your phone
  </p>
  <div class="example">
    <div class="code">
      <CodeEditor
        code={`
    // Android example

    // Retreive buttons from the android application
    val buttonOn = (root.findViewById(R.id.button1) as Button)
    val buttonOff = (root.findViewById(R.id.button2) as Button)

    // Connect arduino with custom connection
    firmata = Firmata(connection);

    // Initialize pin 7 as led
    val led = firmata!!.Led(7);

    // Setup on buttons clicks
    buttonOn.setOnClickListener { led.turnOn() }
    buttonOff.setOnClickListener { led.turnOff() }
        `}
        language="kotlin"
      />
    </div>
    <div class="arrow">
      <div class="right">
        <FaArrowRight />
      </div>
      <div class="down">
        <FaArrowDown />
      </div>
    </div>
    <div class="sketch">
      <ArduinoSketch />
    </div>
  </div>
</div>

<style type="scss">
  div.root {
    margin: 1rem 10rem 1rem 10rem;
    @media only screen and (max-width: 768px) {
      margin: 1rem;
    }
    a {
      font-size: 2em;
      color: white;
      text-decoration: underline;
    }
    div.example {
      display: flex;
      justify-content: space-around;
      align-items: center;
      @media only screen and (max-width: 768px) {
        flex-direction: column;
      }
      div.code {
        width: 40rem;
        @media only screen and (max-width: 768px) {
          width: 100%;
        }
      }
      div.sketch {
        width: 40rem;
        margin-top: 3rem;
        max-width: 100%;
        :global(path#light) {
          display: none;
        }
      }
      div.arrow {
        width: 5rem;
        @media only screen and (max-width: 768px) {
          width: 5rem;
        }
        div.down {
          display: none;
          @media only screen and (max-width: 768px) {
            display: unset;
          }
        }
        div.right {
          @media only screen and (max-width: 768px) {
            display: none;
          }
        }
      }
    }
  }
</style>
