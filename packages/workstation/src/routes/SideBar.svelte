<script lang="ts">
	import Link from '$lib/components/link.svelte';

	let sections = [
		{
			title: '概览',
			open: true,
			items: [
				{ label: '仪表盘', href: '/' },
				{ label: '活动', href: '/activities' }
			]
		},
		{
			title: '资源',
			open: false,
			items: [
				{ label: '任务', href: '/tasks' },
				{ label: '队列', href: '/queues' },
				{ label: '日志', href: '/logs' }
			]
		},
		{
			title: '设置',
			open: false,
			items: [
				{ label: '系统', href: '/settings/system' },
				{ label: '用户', href: '/settings/users' }
			]
		}
	];

	function toggle(idx: number) {
		sections[idx].open = !sections[idx].open;
		sections = sections;
	}
</script>

<aside class="sidebar">
	<div class="brand">
		<span class="brand-dot"></span>
		<span class="brand-text">RMCS Workstation</span>
	</div>

	<nav>
		{#each sections as section, i}
			<div class="section">
				<button class="section-toggle" aria-expanded={section.open} on:click={() => toggle(i)}>
					<span class="toggle-icon" aria-hidden="true">{section.open ? '▾' : '▸'}</span>
					{section.title}
				</button>
				{#if section.open}
					<ul class="menu">
						{#each section.items as item}
							<li>
								<Link href={item.href} class="menu-link">{item.label}</Link>
							</li>
						{/each}
					</ul>
				{/if}
			</div>
		{/each}
	</nav>

	<div class="widgets">
		<div class="widget">
			<div class="widget-title">任务完成率</div>
			<svg viewBox="0 0 120 60" width="100%" height="60" role="img" aria-label="示例折线图">
				<polyline
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					points="0,50 20,40 40,42 60,30 80,22 100,20 120,12"
				/>
			</svg>
		</div>
		<div class="widget">
			<div class="widget-title">队列长度</div>
			<div class="bars">
				<div style="width: 60%" class="bar"></div>
				<div style="width: 35%" class="bar"></div>
				<div style="width: 75%" class="bar"></div>
			</div>
		</div>
	</div>
</aside>

<style>
	.sidebar {
		display: flex;
		flex-direction: column;
		width: 280px;
		min-width: 240px;
		border-right: 1px solid #e5e7eb;
		background: #ffffff;
	}

	.brand {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 16px;
		border-bottom: 1px solid #f1f5f9;
	}

	.brand-dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		background: #f97316; /* flowbite/svelte orange-ish */
	}

	.brand-text {
		font-weight: 600;
	}

	nav {
		padding: 8px 8px 16px 8px;
	}

	.section { margin-bottom: 8px; }

	.section-toggle {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		padding: 10px 12px;
		border-radius: 8px;
		border: 1px solid #e5e7eb;
		background: #fafafa;
		color: #111827;
		cursor: pointer;
	}

	.section-toggle:hover { background: #f1f5f9; }

	.toggle-icon { width: 1rem; }

	.menu {
		list-style: none;
		margin: 6px 0 0 0;
		padding: 0 6px 0 24px;
		display: grid;
		gap: 4px;
	}

	:global(.menu-link) {
		display: inline-block;
		padding: 6px 8px;
		border-radius: 6px;
		color: #334155;
	}

	:global(.menu-link:hover) {
		background: #f8fafc;
		color: #0f172a;
	}

	.widgets {
		margin-top: auto;
		padding: 12px 12px 16px 12px;
		border-top: 1px solid #f1f5f9;
		display: grid;
		gap: 10px;
	}

	.widget { border: 1px solid #e5e7eb; border-radius: 8px; padding: 10px; }
	.widget-title { font-size: 0.9rem; color: #475569; margin-bottom: 6px; }
	.bars { display: grid; gap: 6px; }
	.bar { height: 8px; background: #cbd5e1; border-radius: 999px; }

	@media (max-width: 768px) {
		.sidebar { width: 100%; min-width: 100%; border-right: none; border-bottom: 1px solid #e5e7eb; }
	}
</style>
