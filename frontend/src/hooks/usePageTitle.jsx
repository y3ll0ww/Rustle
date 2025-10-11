import { useEffect } from "react";

export const APP_NAME = "Rustle";

export function usePageTitle(title) {
  useEffect(() => {
    const defaultTitle = APP_NAME;
    document.title = title ? `${title} - ${defaultTitle}` : defaultTitle;
  }, [title]);
}
