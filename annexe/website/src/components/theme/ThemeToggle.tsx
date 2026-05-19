import { Moon, Sun } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useTheme } from "./ThemeProvider";

export const ThemeToggle = ({ className }: { className?: string }) => {
  const { theme, toggle } = useTheme();
  return (
    <Button
      type="button"
      size="icon"
      variant="ghost"
      onClick={toggle}
      aria-label={theme === "dark" ? "Switch to day mode" : "Switch to night mode"}
      className={className}
    >
      <Sun className="size-5 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
      <Moon className="absolute size-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
    </Button>
  );
};
