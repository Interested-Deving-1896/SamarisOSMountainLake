import React from "react";
import { AirBarButton } from "./AirBarButton";

export const AirBarPill = React.memo(function AirBarPill(props: React.ComponentProps<typeof AirBarButton>) {
  return <AirBarButton {...props} />;
});

