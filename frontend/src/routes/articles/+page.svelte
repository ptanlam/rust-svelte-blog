<script>
  // @ts-ignore
  export let data;
  console.log(data);
</script>

<ul>
	{#each data.items as item}
		<li>{item.id} - {item.name}</li>
	{/each}
</ul>