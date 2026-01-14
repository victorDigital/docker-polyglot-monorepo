<script lang="ts">
	import * as InputGroup from '$lib/components/ui/input-group/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import * as Table from '$lib/components/ui/table/index.js';
	import ArrowUpIcon from '@lucide/svelte/icons/arrow-up';
	import { Spinner } from '$lib/components/ui/spinner';
	import { Badge } from '$lib/components/ui/badge';
	import { onMount, onDestroy } from 'svelte';

	type LanguageType = {
		label: string;
		icon: string;
		key: string;
	};

	type Task = {
		taskId: string;
		expression: string;
		language: string;
		sentAt: number;
		status: 'pending' | 'completed';
		result?: string;
		error?: string;
		completedAt?: number;
	};

	const availableLanguages: Record<string, LanguageType> = {
		rust: { label: 'Rust', icon: '🦀', key: 'rust' },
		typescript: { label: 'TypeScript', icon: '🟦', key: 'typescript' },
		python: { label: 'Python', icon: '🐍', key: 'python' }
	};

	let selectedLanguage = $state<LanguageType>({ label: 'Rust', icon: '🦀', key: 'rust' });
	let query = $state<string>('');
	let isSending = $state<boolean>(false);
	let isConnected = $state<boolean>(false);
	let tasks = $state<Task[]>([]);
	let clientId = $state<string>('');
	let eventSource: EventSource | null = null;

	let canSend = $derived(query.trim().length > 0 && !isSending && isConnected);

	// Generate a unique client ID
	function generateClientId(): string {
		return `client-${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;
	}

	// Connect to SSE endpoint for real-time results
	function connectToResults() {
		if (eventSource) {
			eventSource.close();
		}

		eventSource = new EventSource(`/api/results?clientId=${encodeURIComponent(clientId)}`);

		eventSource.onopen = () => {
			console.log('SSE connection opened');
		};

		eventSource.onmessage = (event) => {
			try {
				const data = JSON.parse(event.data);

				if (data.type === 'connected') {
					isConnected = true;
					console.log('Connected to results stream');
				} else if (data.type === 'result') {
					// Update the existing task with the result
					const taskIndex = tasks.findIndex(task => task.taskId === data.taskId);
					if (taskIndex !== -1) {
						tasks[taskIndex] = {
							...tasks[taskIndex],
							status: 'completed',
							result: data.result,
							error: data.error,
							completedAt: data.timestamp
						};
						// Force reactivity by reassigning the array
						tasks = [...tasks];
					}
					console.log('Received result:', data);
				}
			} catch (error) {
				console.error('Error parsing SSE message:', error);
			}
		};

		eventSource.onerror = (error) => {
			console.error('SSE error:', error);
			isConnected = false;
			// Reconnect after a delay
			setTimeout(() => {
				if (clientId) {
					connectToResults();
				}
			}, 3000);
		};
	}

	// Send task to the worker
	async function sendTask() {
		if (!canSend) return;

		const expression = query.trim();
		const language = selectedLanguage.key;

		// Generate task ID upfront
		const taskId = `task-${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;

		isSending = true;

		// Add to tasks as pending immediately
		const newTask = {
			taskId,
			expression,
			language,
			sentAt: Date.now(),
			status: 'pending' as const
		};
		tasks = [newTask, ...tasks];
		query = '';

		try {
			const response = await fetch('/api/tasks', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					expression,
					language,
					clientId,
					taskId // Send the taskId to the server
				})
			});

			const data = await response.json();

			if (data.success) {
				console.log('Task sent:', data.taskId);
			} else {
				console.error('Failed to send task:', data.error);
				// Remove the task if sending failed
				tasks = tasks.filter(t => t.taskId !== taskId);
			}
		} catch (error) {
			console.error('Error sending task:', error);
			// Remove the task if sending failed
			tasks = tasks.filter(t => t.taskId !== taskId);
		} finally {
			isSending = false;
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey && canSend) {
			event.preventDefault();
			sendTask();
		}
	}

	function getLanguageIcon(language: string): string {
		return availableLanguages[language]?.icon ?? '❓';
	}

	onMount(() => {
		clientId = generateClientId();
		connectToResults();
	});

	onDestroy(() => {
		if (eventSource) {
			eventSource.close();
		}
	});
</script>

<div class="container mx-auto max-w-2xl px-4">
	<!-- Connection Status -->
	<div class="fixed top-4 right-4">
		{#if isConnected}
			<Badge variant="default" class="bg-green-600">Connected</Badge>
		{:else}
			<Badge variant="destructive">Disconnected</Badge>
		{/if}
	</div>

	<div class="h-[10vh]"></div>

	<!-- Input Section -->
	<InputGroup.Root class="rounded-3xl">
		<InputGroup.Textarea
			placeholder="Write equation to evaluate (e.g., 2 + 2)"
			bind:value={query}
			onkeydown={handleKeydown}
		/>
		<InputGroup.Addon align="block-end">
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
							onclick={() => (selectedLanguage = lang)}
							class="flex items-center gap-2"
						>
							<span>{lang.icon}</span>
							<span>{lang.label}</span>
						</DropdownMenu.Item>
					{/each}
				</DropdownMenu.Content>
			</DropdownMenu.Root>
			<InputGroup.Button
				variant="default"
				class="ms-auto rounded-full"
				size="icon-xs"
				disabled={!canSend}
				onclick={sendTask}
			>
				{#if isSending}
					<Spinner class="size-4" />
				{:else}
					<ArrowUpIcon />
				{/if}
				<span class="sr-only">Send</span>
			</InputGroup.Button>
		</InputGroup.Addon>
	</InputGroup.Root>

	{#if !isConnected}
		<p class="mt-4 text-center text-sm text-gray-500">
			Connecting to server... Please wait.
		</p>
	{:else if tasks.length > 0}
		<div class="mt-8">
			<h2 class="text-lg font-semibold text-gray-700 dark:text-gray-300 mb-4">Tasks</h2>
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head class="w-3">
							<!-- icon/spinner -->
						</Table.Head>
						<Table.Head>ID</Table.Head>
						<Table.Head>Lang</Table.Head>
						<Table.Head>Req</Table.Head>
						<Table.Head class="text-right">Result</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each tasks as task (task.taskId)}
						<Table.Row>
							<Table.Cell>
								{#if task.status === 'pending'}
									<Spinner class="size-4" />
								{:else}
									<span class="text-green-600 dark:text-green-400">✓</span>
								{/if}
							</Table.Cell>
							<Table.Cell>{task.taskId.slice(-7)}</Table.Cell>
							<Table.Cell>{getLanguageIcon(task.language)}</Table.Cell>
							<Table.Cell>{task.expression}</Table.Cell>
							<Table.Cell class="text-right">
								{#if task.status === 'pending'}
									<span class="italic text-gray-500">Pending...</span>
								{:else if task.error}
									<span class="text-red-600 italic">Error: {task.error}</span>
								{:else}
									{task.result}
								{/if}
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</div>
	{/if}
</div>
