<script lang="ts">
	import * as InputGroup from '$lib/components/ui/input-group/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import IconPlus from '@lucide/svelte/icons/plus';
	import ArrowUpIcon from '@lucide/svelte/icons/arrow-up';
	import { Separator } from '$lib/components/ui/separator';

  type LanguageType = {
    label: string;
    icon: string;
  } 

  const availableLanguages: Record<string, LanguageType> = {
    rust: { label: 'Rust', icon: '🦀' },
    typescript: { label: 'TypeScript', icon: '🟦' },
    python: { label: 'Python', icon: '🐍' }
  }

  let selectedLanguage = $state<LanguageType>({ label: 'Rust', icon: '🦀' });

  let query = $state<string>('');

  let canSend = $derived(query.trim().length > 0);

  function send(query: string, selectedLanguage: LanguageType) {
    return () => {
      console.log(`Sending query: ${query} in language: ${selectedLanguage.label}`);
      // Implement the actual send logic here
    }
  }
</script>

<div class="container mx-auto max-w-2xl">
	<div class="h-[25vh]"></div>
	<InputGroup.Root class="rounded-3xl" >
		<InputGroup.Textarea placeholder="Write equation to evaluate" bind:value={query}/>
		<InputGroup.Addon align="block-end">
			<!-- <InputGroup.Button variant="outline" class="rounded-full" size="icon-xs">
				<IconPlus />
			</InputGroup.Button> -->
			<DropdownMenu.Root>
				<DropdownMenu.Trigger>
					{#snippet child({ props })}
          <InputGroup.Button class="flex items-center gap-1" {...props} variant="outline">
            <span>{selectedLanguage.icon}</span>
            <span>{selectedLanguage.label}</span>
          </InputGroup.Button>
					{/snippet}
				</DropdownMenu.Trigger>
				<DropdownMenu.Content side="top" align="start">
					{#each Object.entries(availableLanguages) as [key, lang]}
            <DropdownMenu.Item 
              onclick={() => selectedLanguage = lang} 
              class="flex items-center gap-2"
            >
              <span>{lang.icon}</span>
              <span>{lang.label}</span>
            </DropdownMenu.Item>
          {/each}
				</DropdownMenu.Content>
			</DropdownMenu.Root>
			<!-- <InputGroup.Text class="ms-auto">52% used</InputGroup.Text>
			<Separator orientation="vertical" class="h-4!" /> -->
			<InputGroup.Button variant="default" class="ms-auto rounded-full" size="icon-xs" disabled={!canSend} onclick={send(query, selectedLanguage)}>
				<ArrowUpIcon />
				<span class="sr-only">Send</span>
			</InputGroup.Button>
		</InputGroup.Addon>
	</InputGroup.Root>
</div>
