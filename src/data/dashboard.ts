import type { LucideIcon } from 'lucide-react';
import { Database, Files, FolderKanban, History, LayoutDashboard } from 'lucide-react';
import type { View } from '@/types/view';

export interface NavItem {
  id: View;
  label: string;
  icon: LucideIcon;
}

export const navItems: NavItem[] = [
  { id: 'overview', label: 'Overview', icon: LayoutDashboard },
  { id: 'caches', label: 'Developer caches', icon: Database },
  { id: 'projects', label: 'Projects', icon: FolderKanban },
  { id: 'files', label: 'Large files', icon: Files },
  { id: 'history', label: 'History', icon: History },
];

export const viewCopy: Record<View, { title: string; description: string }> = {
  overview: {
    title: 'Overview',
    description: 'Scan your workspace and review storage health at a glance.',
  },
  caches: {
    title: 'Developer caches',
    description: 'Review rebuildable toolchain data before moving anything to Trash.',
  },
  projects: {
    title: 'Projects',
    description: 'Find stale build artifacts inside your repositories.',
  },
  files: {
    title: 'Large files',
    description: 'Inspect the largest files in your workspace before taking action.',
  },
  history: {
    title: 'History',
    description: 'See what Cachebin has moved to Trash and when.',
  },
  settings: {
    title: 'Settings',
    description: 'Control scan behavior, cleanup safeguards, and local preferences.',
  },
};
