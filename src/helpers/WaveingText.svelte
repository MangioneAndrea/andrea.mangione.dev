<script>
  export let text= "";
  const timePerLetter=2000/text.length;

  const elems = Array.from(text).map((letter, index) => ({letter, delay: index*timePerLetter}));
  
</script>

{#each elems as {letter, delay}}
	<span style={`--delay: ${delay}ms;`}>{letter}</span>
{/each}

<style>
	@keyframes wave-text {
	  00% {
	    transform: translateY(0em);
	  }
	  60% {
	    transform: translateY(-0.5em);
	  }
	  100% {
	    transform: translateY(0em);
	  }
	}
	span{
            display: inline-block;
	    animation: wave-text 2s ease-in-out infinite;
	    animation-delay: var(--delay);
	}
</style>
