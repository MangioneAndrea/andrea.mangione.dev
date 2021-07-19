<script>
  import Stack from "../../elements/stack/Stack.svelte";
  import React from "../../images/Logos/React.svg";
  import Angular from "../../images/Logos/Angular.svg";
  import Svelte from "../../images/Logos/Svelte.svg";
  import Kotlin from "../../images/Logos/Kotlin.svg";
  import Java from "../../images/Logos/Java.svg";
  import NodeJS from "../../images/Logos/NodeJS.svg";
  import Go from "../../images/Logos/Go.svg";
  import MongoDB from "../../images/Logos/MongoDB.svg";
  import GraphQL from "../../images/Logos/GraphQL.svg";
  import Unreal from "../../images/Logos/UnrealEngine.svg";
  import SingleLanguages from "../../elements/stack/SingleLanguages.svelte";
</script>

<div id="stacks">
  <h1>My Stacks</h1>
  <Stack
    title="Web apps | Frontend"
    description="Web-developing's is getting more popular every day. Today there are 
    countless frameworks one can use. From the very standard svelte to the more peculiar Vue!"
    languages={[
      {
        image: React,
        key: "react",
        description:
          "React has it's best advantage when creating complex yet big webapps. The hook's" +
          " feature made it very simple to write reusable components",
      },
      {
        image: Angular,
        key: "angular",
        description:
          "Angular is my preference with bigger projects. It gets easyer to work in bigger" +
          " groups if you have such a great structure",
      },
      {
        image: Svelte,
        key: "svelte",
        description:
          "Svelte is fantastic for small/medium webapps. It reduces the amount of code and complexity" +
          " by a lot. This page has been made with svelte :D",
      },
    ]}
  />

  <Stack
    title="Standalones | Backend"
    description="Structure, security, reliability. That is what each app needs. One good language can allow this and even more"
    languages={[
      {
        image: Go,
        key: "Golang",
        description:
          "Golang is my newest favourite language. It's just beautiful and needs very less effort " +
          "to be deployed. Dividing a monolith into microservices has never been so easy before",
      },
      {
        image: Kotlin,
        key: "Kotlin",
        description:
          "Kotlin is just java but better. All reduction of the code made this my preferite Object" +
          " Oriented language ever",
      },
      {
        image: NodeJS,
        key: "Node",
        description:
          "NodeJS has in my opinion the best community. It is just perfect for small/mid projects " +
          " and can unify all the developer by letting them use only one language!",
      },
    ]}
  />
  <SingleLanguages
    title="Honorable mentions"
    description="The stacks I've been latelly using, do not belong directly to frontend/backend, but surelly deserve to be mentioned"
    languages={[
      {
        image: MongoDB,
        key: "MongoDB",
        description:
          "Mongo is my all time favourite database type. It's very powerful and even the worst structured databased can be very fast thanks to the aggregation framework",
      },
      {
        image: GraphQL,
        key: "GraphQL",
        description:
          "GraphQL is the easiest way to let front and backend communicate. The structure is very simple and the error handling is just the best. More complicated features like scalars can make the communication super clean.",
      },
      {
        image: Unreal,
        key: "Unreal",
        description:
          "Unreal is my preferite game engine. The combination bewtween the performance and cotrol of C++ and the prototyping of Blueprint, it is possible to create cool games even as a solo developer",
      },
    ]}
  />
</div>

<style type="text/scss">
  div#stacks {
    margin: auto;
    width: 100%;
    text-align: center;
    h1 {
      font-size: 4.5rem;
      margin: auto;
    }
  }
</style>
