<script>
    import ArduinoSketch from "./ArduinoSketch.svelte";
    import CodeEditor from "../../elements/codeEditor/CodeEditor.svelte";
    import LanguageList from "../../elements/containers/LanguageList.svelte";
</script>

<div class="root">
    <a href="https://github.com/MangioneAndrea/kotlin-firmata" target="_blank" class="projectTitle">
        <h2>Kotlin firmata </h2>
    </a>
    <LanguageList icons={["assets/Logos/KotlinBig.svg", "assets/Logos/Arduino.svg"]}/>
    <p>
        This project helps you create arduino projects without c knowledge allowing
        multiplatforming. As kotlin is the main language for android native apps, it
        also helps you develop prototypes guided by your phone
    </p>
    <div class="example">
        <div class="code">
            <CodeEditor
                    code={`// Android example
    
    // Retreive switch from the android application
    val switch = (root.findViewById(R.id.switch) as Switch)

    // Connect arduino with custom connection
    firmata = Firmata(connection);

    // Initialize pin 7 as led
    val led = firmata!!.PWMLed(7);

    // switch logic
    switch.setOnCheckedChangeListener { _, isChecked ->
        if (isChecked) led.turnOn() else led.turnOff()
    } `}
                    language="kotlin"
            />
        </div>
        <div class="sketch">
            <ArduinoSketch/>
        </div>
    </div>
</div>

<style>
    div.root {
        margin: 1rem 3rem 1rem 3rem;
    }

    @media only screen and (max-width: 768px) {
        div.root {
            margin: 1rem;
        }
    }


    div.example {
        display: flex;
        justify-content: space-around;
        align-items: center;
    }

    @media (max-width: 1366px) {
        div.example {
            flex-direction: column;
        }
    }

    div.code {
        width: 40rem;
    }

    @media only screen and (max-width: 768px) {
        div.code {
            width: 100%;
        }
    }

    div.sketch {
        width: 40rem;
        margin-top: 3rem;
        max-width: 100%;
    }

    :global(path#light) {
        display: none;
    }


</style>
