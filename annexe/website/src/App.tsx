import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { lazy, Suspense } from "react";
import { Toaster as Sonner } from "@/components/ui/sonner";
import { Toaster } from "@/components/ui/toaster";
import { TooltipProvider } from "@/components/ui/tooltip";
import { Layout } from "@/components/layout/Layout";
import { ThemeProvider } from "@/components/theme/ThemeProvider";
import { Analytics } from "@vercel/analytics/react";
import { SpeedInsights } from "@vercel/speed-insights/react";
import Index from "./pages/Index.tsx";

const Software = lazy(() => import("./pages/Software.tsx"));
const Download = lazy(() => import("./pages/Download.tsx"));
const Faq = lazy(() => import("./pages/Faq.tsx"));
const Business = lazy(() => import("./pages/Business.tsx"));
const NotFound = lazy(() => import("./pages/NotFound.tsx"));
const License = lazy(() => import("./pages/License.tsx"));
const Interface = lazy(() => import("./pages/Interface.tsx"));

const queryClient = new QueryClient();

const App = () => (
  <QueryClientProvider client={queryClient}>
    <ThemeProvider>
      <TooltipProvider>
        <Toaster />
        <Sonner />
        <BrowserRouter>
          <Routes>
            <Route element={<Layout />}>
              <Route path="/" element={<Index />} />
              <Route
                path="/software"
                element={
                  <Suspense fallback={null}>
                    <Software />
                  </Suspense>
                }
              />
              <Route
                path="/interface"
                element={
                  <Suspense fallback={null}>
                    <Interface />
                  </Suspense>
                }
              />
              <Route
                path="/download"
                element={
                  <Suspense fallback={null}>
                    <Download />
                  </Suspense>
                }
              />
              <Route
                path="/license"
                element={
                  <Suspense fallback={null}>
                    <License />
                  </Suspense>
                }
              />
              <Route
                path="/faq"
                element={
                  <Suspense fallback={null}>
                    <Faq />
                  </Suspense>
                }
              />
              <Route
                path="/business"
                element={
                  <Suspense fallback={null}>
                    <Business />
                  </Suspense>
                }
              />
              <Route
                path="*"
                element={
                  <Suspense fallback={null}>
                    <NotFound />
                  </Suspense>
                }
              />
            </Route>
          </Routes>
        </BrowserRouter>
        <Analytics />
        <SpeedInsights />
      </TooltipProvider>
    </ThemeProvider>
  </QueryClientProvider>
);

export default App;
