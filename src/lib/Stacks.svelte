<script>
    import Stack from "../elements/Stack.svelte";
</script>

<div id="stacks">
    <h1>My Stacks</h1>
    <Stack
            description="Web frameworks"
            languages={[
              {
                image: "assets/Logos/React.svg",
                key: "react",
              },
              {
                image: "assets/Logos/Angular.svg",
                key: "angular",
              },
              {
                image: "assets/Logos/Svelte.svg",
                key: "svelte",
              },
            ]}
    />

    <Stack
            description="Server/Standalone"
            languages={[
              {
                image: "assets/Logos/Go.svg",
                key: "Golang",
              },
              {
                image: "assets/Logos/Kotlin.svg",
                key: "Kotlin",
              },
              {
                image: "assets/Logos/NodeJS.svg",
                key: "Node",
              },
            ]}
    />
</div>

<style>
    div#stacks {
        margin: auto;
        width: 100%;
        text-align: center;
    }

    div#stacks > h1 {
        font-size: 4.5rem;
        margin: auto;
    }
</style>
