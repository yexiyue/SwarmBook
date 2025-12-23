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
					label: 'libp2p 核心概念',
					translations: { en: 'Core Concepts' },
					autogenerate: { directory: '01-core-concepts' },
				},
				{
					label: '传输层与连接',
					translations: { en: 'Transport & Connection' },
					autogenerate: { directory: '02-transport' },
				},
				{
					label: '协议与流',
					translations: { en: 'Protocols & Streams' },
					autogenerate: { directory: '03-protocols' },
				},
				{
					label: '节点发现',
					translations: { en: 'Discovery' },
					autogenerate: { directory: '04-discovery' },
				},
				{
					label: '消息传播',
					translations: { en: 'PubSub' },
					autogenerate: { directory: '05-pubsub' },
				},
				{
					label: '生产环境',
					translations: { en: 'Production' },
					autogenerate: { directory: '06-production' },
				},
				{
					label: 'P2P 协作编辑',
					translations: { en: 'P2P Collaboration' },
					badge: {
						text: { 'zh-CN': '实战', en: 'Hands-on' },
						variant: 'success'
					},
					autogenerate: { directory: '07-yjs-collab' },
				},
			],
		}),
	],
	base: "/SwarmBook"
});
