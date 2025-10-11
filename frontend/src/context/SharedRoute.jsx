import { usePageTitle } from "../hooks/usePageTitle";

export default function SharedRoute({ title, children }) {
  usePageTitle(title);
  return children;
};
