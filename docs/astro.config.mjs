// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import mermaid from 'astro-mermaid';

// https://astro.build/config
export default defineConfig({
	integrations: [
		mermaid(),
		starlight({
			title: 'SwarmBook',
			description: '和我一起探索去中心化网络的奥秘 - Rust libp2p 学习笔记与实战',
			defaultLocale: 'root',
			locales: {
				root: { label: '简体中文', lang: 'zh-CN' },
				en: { label: 'English', lang: 'en' },
			},
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/yexiyue/SwarmBook' }],
			sidebar: [
				{
					label: 'P2P 基础概念',
					translations: { en: 'P2P Basics' },
					autogenerate: { directory: '01-p2p-basics' },
				},
				{
					label: '传输层与安全',
					translations: { en: 'Transport & Security' },
					autogenerate: { directory: '02-transport-security' },
				},
				{
					label: 'Swarm 与协议',
					translations: { en: 'Swarm & Protocols' },
					autogenerate: { directory: '03-swarm-protocols' },
				},
				{
					label: '节点发现与路由',
					translations: { en: 'Discovery & Routing' },
					autogenerate: { directory: '04-discovery-routing' },
				},
				{
					label: '消息传播',
					translations: { en: 'Messaging' },
					autogenerate: { directory: '05-messaging' },
				},
				{
					label: 'P2P yjs 后端',
					translations: { en: 'P2P yjs Backend' },
					badge: {
						text: { 'zh-CN': '实战', en: 'Hands-on' },
						variant: 'success'
					},
					autogenerate: { directory: '06-yjs-backend' },
				},
			],
		}),
	],
	base: "/SwarmBook"
});
