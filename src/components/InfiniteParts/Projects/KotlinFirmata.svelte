<script>
  import CodeEditor from "../../../elements/codeEditor/CodeEditor.svelte";
  import LanguageList from "../../../elements/containers/LanguageList.svelte";
  import Kotlin from "../../../images/Logos/Kotlin.svg";
  import FaArrowRight from "svelte-icons/fa/FaArrowRight.svelte";
  import FaArrowDown from "svelte-icons/fa/FaArrowDown.svelte";
</script>

<div class="root">
  <h2>Kotlin firmata</h2>
  <LanguageList icons={[Kotlin]} />
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

    // Initialize pin 11 as led
    val led = firmata!!.PWMLed(11);

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
  </div>
</div>

<style type="scss">
  div.root {
    margin: 1rem 10rem 1rem 10rem;
    @media only screen and (max-width: 768px) {
      margin: 1rem;
    }
    h2 {
      margin-bottom: 0;
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
      div.arrow {
        width: 5rem;
        @media only screen and (max-width: 768px) {
          width: 5rem;
        }
        display: none;
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
