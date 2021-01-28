<script>
  import Stack from "../../elements/stack/Stack.svelte";
  import React from "../../images/Logos/React.svg";
  import Angular from "../../images/Logos/Angular.svg";
  import Svelte from "../../images/Logos/Svelte.svg";
  import Kotlin from "../../images/Logos/Kotlin.svg";
  import Java from "../../images/Logos/Java.svg";
  import NodeJS from "../../images/Logos/NodeJS.svg";
  import MongoDB from "../../images/Logos/MongoDB.svg";
  import GraphQL from "../../images/Logos/GraphQL.svg";
  import Unreal from "../../images/Logos/UnrealEngine.svg";
  import SingleLanguages from "../../elements/stack/SingleLanguages.svelte";
</script>

<div id="stacks">
  <Stack
    title="Web apps | Frontend"
    languages={[
      {
        image: React,
        description: "react",
      },
      {
        image: Angular,
        description: "angular",
      },
      {
        image: Svelte,
        description: "svelte",
      },
    ]}
  />
  <Stack
    title="Standalones | Backend"
    languages={[
      {
        image: Java,
        description: "Java",
      },
      {
        image: Kotlin,
        description: "Kotlin",
      },
      {
        image: NodeJS,
        description: "Node",
      },
    ]}
  />
  <SingleLanguages
    title="Honorable mentions"
    languages={[
      {
        image: MongoDB,
        description: "MongoDB",
      },
      {
        image: GraphQL,
        description: "GraphQL",
      },
      {
        image: Unreal,
        description: "Unreal",
      },
    ]}
  />
</div>

<style type="text/scss">
  div#stacks {
  }
</style>
