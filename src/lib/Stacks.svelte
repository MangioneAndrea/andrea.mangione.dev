<script>
  import Stack from "../elements/Stack.svelte";
</script>

<section>
  <h1>My Stack</h1>
  <p>
    Nowadays there are tons of technologies we can make use of. I tried to cover
    all the range that I need to do the job.
  </p>
  <div>
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
        {
          image: "assets/Logos/Rust.svg",
          key: "asd",
        },
      ]}
    />
  </div>
  <div>
    <Stack
      description="Others"
      languages={[
        {
          image: "assets/Logos/GraphQL.svg",
          key: "graphql",
        },
        {
          image: "assets/Logos/MongoDB.svg",
          key: "mongo",
        },
        {
          image: "assets/Logos/Grpc.svg",
          key: "grpc",
        },
      ]}
    />
  </div>
</section>

<style>
  section > div {
    display: flex;
    justify-content: space-evenly;
    max-width: var(--max-width);
    margin: auto;
  }

  @media only screen and (max-width: 1366px) {
    section > div {
      flex-direction: column;
    }
  }

  h1 {
    background: var(--blueish-background);
    -webkit-background-clip: text;
    background-clip: text;
  }
</style>
