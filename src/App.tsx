import { lazy, Suspense, useEffect, useRef, useState } from 'react';
import { toast } from 'sonner';
import { AppHeader } from '@/components/AppHeader';
import { AppSidebar } from '@/components/AppSidebar';
import { CompactPopup } from '@/components/CompactPopup';
import { PageFallback } from '@/components/PageFallback';
import { TitleBar } from '@/components/TitleBar';
import { TooltipProvider } from '@/components/ui/tooltip';
import { Toaster } from '@/components/ui/sonner';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { viewCopy } from '@/data/dashboard';
import { useHistory } from '@/hooks/useHistory';
import { useScan } from '@/hooks/useScan';
import { useSettings } from '@/hooks/useSettings';
import { useZoomShortcuts } from '@/hooks/useZoomShortcuts';
import type { View } from '@/types/view';
import './App.css';

const OverviewPage = lazy(() => import('@/components/OverviewPage').then((m) => ({ default: m.OverviewPage })));
const CachesPage = lazy(() => import('@/components/CachesPage').then((m) => ({ default: m.CachesPage })));
const ProjectsPage = lazy(() => import('@/components/ProjectsPage').then((m) => ({ default: m.ProjectsPage })));
const LargeFilesPage = lazy(() => import('@/components/LargeFilesPage').then((m) => ({ default: m.LargeFilesPage })));
const HistoryPage = lazy(() => import('@/components/HistoryPage').then((m) => ({ default: m.HistoryPage })));
const SettingsPage = lazy(() => import('@/components/SettingsPage').then((m) => ({ default: m.SettingsPage })));
const PlaceholderView = lazy(() =>
  import('@/components/PlaceholderView').then((m) => ({ default: m.PlaceholderView })),
);

const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

function App() {
  const label = isTauri ? getCurrentWindow().label : 'main';
  return label === 'tray-popup' ? <CompactPopup /> : <MainApp />;
}

function MainApp() {
  const [activeView, setActiveView] = useState<View>('overview');
  const [cacheCategory, setCacheCategory] = useState('all');
  const [mobileNavOpen, setMobileNavOpen] = useState(false);
  const { platform, result, scanState, progress, lastScanAt, rescan } = useScan();
  const { history, recordScanStart, completeScan, recordCleanup, clearHistory } = useHistory();
  const { settings, scanDirOptions, update: updateSettings } = useSettings();
  const autoScanned = useRef(false);

  useZoomShortcuts();

  const selectView = (view: View) => {
    setActiveView(view);
    setCacheCategory('all');
    setMobileNavOpen(false);
  };

  const openCategory = (category: string) => {
    setCacheCategory(category);
    setActiveView('caches');
  };

  const handleScan = async () => {
    const id = recordScanStart();
    const bytes = await rescan();
    completeScan(id, bytes === null ? 'error' : 'success', bytes ?? 0);
  };

  const handleScanRef = useRef(handleScan);
  handleScanRef.current = handleScan;

  useEffect(() => {
    if (settings?.autoScanOnStartup && !autoScanned.current) {
      autoScanned.current = true;
      void handleScanRef.current();
    }
  }, [settings, autoScanned]);

  const handleCleanup = (bytes: number) => {
    recordCleanup(bytes);
    void rescan();
  };

  const handleClearHistory = () => {
    clearHistory();
    toast.success('Activity history cleared.');
  };

  const pageTitle = viewCopy[activeView].title;

  return (
    <TooltipProvider delayDuration={300}>
      <div className="flex h-screen flex-col overflow-hidden bg-background text-foreground">
        <TitleBar />
        <div className="flex min-h-0 flex-1">
          <AppSidebar
            activeView={activeView}
            mobileNavOpen={mobileNavOpen}
            onSelectView={selectView}
            onClose={() => setMobileNavOpen(false)}
          />
          {mobileNavOpen && (
            <button
              className="fixed inset-0 z-10 border-0 bg-black/30"
              type="button"
              aria-label="Close navigation"
              onClick={() => setMobileNavOpen(false)}
            />
          )}

          <div className="flex min-w-0 flex-1 flex-col">
            <AppHeader
              title={pageTitle}
              scanState={scanState}
              scanProgress={progress}
              onScan={handleScan}
              onOpenNavigation={() => setMobileNavOpen(true)}
            />
            <main className="min-h-0 flex-1 overflow-y-auto px-[clamp(20px,4vw,48px)] pb-14 pt-[38px] max-[720px]:px-5 max-[720px]:pb-11 max-[720px]:pt-7">
              <Suspense fallback={<PageFallback />}>
                {activeView === 'overview' ? (
                  <OverviewPage
                    platform={platform}
                    result={result}
                    scanState={scanState}
                    lastScanAt={lastScanAt}
                    history={history}
                    onReview={() => selectView('caches')}
                    onOpenHistory={() => selectView('history')}
                    onOpenCategory={openCategory}
                  />
                ) : activeView === 'caches' ? (
                  <CachesPage
                    entries={result?.entries ?? []}
                    scanState={scanState}
                    category={cacheCategory}
                    defaultRisk={settings?.defaultRiskFilter ?? 'all'}
                    onCategoryChange={setCacheCategory}
                    onCleanup={handleCleanup}
                  />
                ) : activeView === 'projects' ? (
                  <ProjectsPage onCleanup={handleCleanup} />
                ) : activeView === 'files' ? (
                  <LargeFilesPage onCleanup={handleCleanup} />
                ) : activeView === 'history' ? (
                  <HistoryPage history={history} />
                ) : activeView === 'settings' ? (
                  <SettingsPage
                    settings={settings}
                    scanDirOptions={scanDirOptions}
                    onUpdate={updateSettings}
                    onClearHistory={handleClearHistory}
                  />
                ) : (
                  <PlaceholderView view={activeView} />
                )}
              </Suspense>
            </main>
          </div>

          <Toaster position="bottom-right" />
        </div>
      </div>
    </TooltipProvider>
  );
}

export default App;
