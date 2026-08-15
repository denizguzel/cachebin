import { useState } from 'react';
import { FolderOpen, FolderKanban } from 'lucide-react';
import { CacheReviewDialog } from '@/components/CacheReviewDialog';
import { ProjectGroupCard } from '@/components/ProjectGroupCard';
import { SelectionBar } from '@/components/SelectionBar';
import { Checkbox } from '@/components/ui/checkbox';
import { useCleanupSelection } from '@/hooks/useCleanupSelection';
import { useTauriQuery } from '@/hooks/useTauriQuery';
import { sourceLabel } from '@/lib/environment';
import { formatBytes } from '@/lib/format';
import type { ProjectArtifact } from '@/types/project-artifact';
import type { ProjectGroup } from '@/types/project-group';

export interface ProjectsPageProps {
  onCleanup: (bytes: number) => void;
}

export function ProjectsPage({ onCleanup }: ProjectsPageProps) {
  const query = useTauriQuery<undefined, ProjectArtifact[]>({ command: 'scan_projects' });
  const [reviewOpen, setReviewOpen] = useState(false);
  const artifacts = query.data ?? [];
  const { selectedIds, selected, selectedBytes, progress, toggle, toggleAll, clear, moveSelectedToTrash } =
    useCleanupSelection({ items: artifacts, onCleanup });

  const groups = groupProjects(artifacts);

  if (query.isPending && !query.data) {
    return (
      <div className="mx-auto flex min-h-[40vh] w-full max-w-[1320px] flex-col items-center justify-center gap-3 text-center">
        <FolderKanban className="animate-pulse text-muted-tertiary" size={28} />
        <p className="text-sm font-medium">Scanning projects…</p>
        <p className="max-w-[420px] text-xs leading-5 text-muted-tertiary">
          Looking for build artifacts inside your repositories.
        </p>
      </div>
    );
  }

  if (query.error) {
    return (
      <div className="mx-auto flex min-h-[220px] w-full max-w-[1320px] flex-col items-center justify-center gap-2.5 text-center">
        <p className="text-sm font-medium">Project scan failed</p>
        <p className="max-w-[420px] text-xs leading-5 text-muted-tertiary">{String(query.error)}</p>
      </div>
    );
  }

  if (groups.length === 0) {
    return (
      <div className="mx-auto flex min-h-[220px] w-full max-w-[1320px] flex-col items-center justify-center gap-2.5 text-center">
        <FolderOpen size={18} className="shrink-0 text-muted-tertiary" />
        <p className="m-0 text-sm font-medium">No project artifacts found</p>
        <p className="max-w-[420px] text-xs leading-5 text-muted-tertiary">
          Cachebin looks in Documents, Desktop, Projects, and similar workspace folders.
        </p>
      </div>
    );
  }

  return (
    <div className="mx-auto w-full max-w-[1320px]">
      <div className="flex items-start justify-between gap-6 pb-6 max-[720px]:flex-col max-[720px]:gap-3">
        <div className="min-w-0">
          <p className="text-[10px] font-medium uppercase tracking-[0.08em] text-muted-tertiary">
            Build artifacts inside your repositories
          </p>
          <p className="mt-[5px] max-w-[560px] text-xs leading-5 text-muted-tertiary">
            Stale build and tooling artifacts grouped by project. Review items individually or as a project before
            moving them to Trash.
          </p>
        </div>
        <div className="flex shrink-0 items-center gap-3">
          <Checkbox
            checked={artifacts.length > 0 && artifacts.every((item) => selectedIds.has(item.id))}
            onCheckedChange={() => toggleAll(artifacts.map((item) => item.id))}
            aria-label="Select all artifacts"
          />
          <span className="text-xs text-muted-tertiary">
            {formatBytes(artifacts.reduce((sum, item) => sum + item.sizeBytes, 0))} cleanable
          </span>
        </div>
      </div>

      <div className="space-y-8">
        {groups.map((group) => (
          <ProjectGroupCard
            group={group}
            key={group.projectPath}
            selectedIds={selectedIds}
            onToggle={toggle}
            onToggleAll={toggleAll}
          />
        ))}
      </div>

      {selectedIds.size > 0 && (
        <SelectionBar
          count={selectedIds.size}
          bytes={selectedBytes}
          onClear={clear}
          onReview={() => setReviewOpen(true)}
        />
      )}

      <CacheReviewDialog
        open={reviewOpen}
        items={selected}
        progress={progress}
        onClose={() => setReviewOpen(false)}
        onConfirm={moveSelectedToTrash}
      />
    </div>
  );
}

function groupProjects(artifacts: ProjectArtifact[]): ProjectGroup[] {
  const byPath = new Map<string, ProjectArtifact[]>();
  for (const artifact of artifacts) {
    const list = byPath.get(artifact.projectPath) ?? [];
    list.push(artifact);
    byPath.set(artifact.projectPath, list);
  }

  return [...byPath.entries()]
    .map(([projectPath, items]) => ({
      projectPath,
      projectName: projectPath.split(/[\\/]/).filter(Boolean).pop() ?? projectPath,
      source: items[0] ? sourceLabel(items[0].environment) : 'Windows',
      items,
      totalBytes: items.reduce((sum, item) => sum + item.sizeBytes, 0),
    }))
    .sort((a, b) => b.totalBytes - a.totalBytes);
}
