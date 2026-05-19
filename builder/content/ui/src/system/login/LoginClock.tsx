import React from "react";

export function LoginClock() {
  const [now, setNow] = React.useState(() => new Date());

  React.useEffect(() => {
    const id = window.setInterval(() => setNow(new Date()), 1000);
    return () => window.clearInterval(id);
  }, []);

  return (
    <div className="samaris-login__clock" aria-live="polite">
      <div className="samaris-login__time">
        {now.toLocaleTimeString([], {
          hour: "2-digit",
          minute: "2-digit",
          hour12: false
        })}
      </div>
      <div className="samaris-login__date">
        {now.toLocaleDateString("en-US", {
          weekday: "long",
          month: "long",
          day: "numeric"
        })}
      </div>
    </div>
  );
}
