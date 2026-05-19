import React from "react";
import { BenchDashboard } from "./bench/components/BenchDashboard";
import "./bench/styles/bench.css";

export default function Bench(_props: { windowId: string }) {
  return <BenchDashboard />;
}
