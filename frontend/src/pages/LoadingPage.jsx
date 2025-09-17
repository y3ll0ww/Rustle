import "../assets/loading.css";

export default function LoadingPage() {
  return (
    <svg
      className="pl"
      viewBox="0 0 420 420"
      width="420px"
      height="420px"
      role="img"
      aria-label="Loading"
    >
      <defs>
        <symbol id="brick">
          <polygon fill="var(--brick-top)" points="70 0,140 39,70 125,0 39" />
          <polygon fill="var(--brick-left)" points="0 39,70 78,71 78,70 125,0 86" />
          <polygon points="70 78,140 39,140 86,70 125" />
          <use href="#studs" />
        </symbol>
        <symbol id="stud">
          <rect y="1" rx="14" ry="8" width="28" height="24" />
          <ellipse fill="var(--brick-top)" cx="14" cy="8" rx="14" ry="8" />
        </symbol>
        <symbol id="studs">
          <use href="#stud" transform="translate(21, 22)" />
          <use href="#stud" transform="translate(56, 3)" />
          <use href="#stud" transform="translate(56, 41)" />
          <use href="#stud" transform="translate(91, 22)" />
        </symbol>
      </defs>
      <g transform="translate(140, 218)">
        <g className="pl__brick-group">
          <g className="pl__brick-group pl__brick-group--1">
            <use className="pl__brick pl__brick--blue" href="#brick" />
            <g className="pl__brick-group pl__brick-group--2" transform="translate(0, -47)">
              <use className="pl__brick pl__brick--green" href="#brick" />
              <g className="pl__brick-group pl__brick-group--3" transform="translate(0, -47)">
                <use className="pl__brick pl__brick--orange" href="#brick" />
                <g className="pl__brick-group pl__brick-group--4" transform="translate(0, -47)">
                  <use className="pl__brick pl__brick--red" href="#brick" />
                </g>
              </g>
            </g>
          </g>
        </g>
      </g>
    </svg>
  );
}
