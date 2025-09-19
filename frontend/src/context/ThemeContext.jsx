import React, { createContext, useContext, useEffect, useState } from "react";
import LogoDark from "../assets/logo-dark.png";
import LogoLight from "../assets/logo-light.png";
import LogoDarkIcon from "../assets/logo-dark-icon.png";
import LogoLightIcon from "../assets/logo-light-icon.png";

const ThemeContext = createContext();

export const Theme = {
    dark: "dark",
    light: "light",
};

export function ThemeProvider({ children }) {
  const [theme, setTheme] = useState(() => {
    // Load saved theme or default to "dark"
    return localStorage.getItem("theme") || Theme.dark;
  });

  // Apply theme to <html> and persist it
  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }, [theme]);

  function toggleTheme() {
    setTheme((prev) => (prev === Theme.dark ? Theme.light : Theme.dark));
  }

  function logo() {
    return theme === Theme.dark ? LogoLight : LogoDark;
  }

  function logoIcon(light, dark) {
    return theme === Theme.dark ? LogoLightIcon : LogoDarkIcon;
  }

  return (
    <ThemeContext.Provider value={{ theme, setTheme, toggleTheme, logo, logoIcon }}>
      {children}
    </ThemeContext.Provider>
  );
}

export function useTheme() {
  return useContext(ThemeContext);
}
