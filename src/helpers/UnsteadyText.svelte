<script>
    import { oneRandomOutOf } from "./CommonFunctions";


  export let text= "";

 let elems = Array.from(text).map((letter) => ({letter, where: oneRandomOutOf("0.5em", "-0.5em")}));

</script>

{#each elems as {letter, where}}
	<span style={`--where: translateY(${where});`}>{letter}</span>
{/each}

<style>
	span{
		display: inline-block;
		transition: all ease-in-out 0.5s;
	}
	span:hover{
		transform: var(--where);
	}
</style>
