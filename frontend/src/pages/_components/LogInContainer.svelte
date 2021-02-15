<test>
  <div class="top" />
  <div class="login">
    <h1>Login</h1>
    <button class="btn github">Github</button>
    <button class="btn gmail">Gmail</button>
    <button class="btn smyto">Smyto</button>
  </div>
  <div class="bottom">
    <svg
      version="1.1"
      id="cloud"
      xmlns="http://www.w3.org/2000/svg"
      xmlns:xlink="http://www.w3.org/1999/xlink"
      x="0px"
      y="90px"
      width="65px"
      height="140px"
      viewBox="0 0 500 500"
      style="enable-background:new 0 0 453 453;"
      xml:space="preserve"
    >
      <g>
        <path
          d="M348.533,187.944c-2.287,0-4.596,0.084-6.926,0.241c-5.621-73.995-67.623-132.488-143.048-132.488
		c-79.095,0-143.45,64.348-143.45,143.454c0,0.47,0.004,0.935,0.018,1.406C21.396,218.757,0,254.172,0,292.832
		c0,57.833,47.046,104.893,104.885,104.893h243.648c57.828,0,104.888-47.055,104.888-104.893
		C453.421,234.997,406.361,187.944,348.533,187.944z M348.533,375.305H104.885c-45.472,0-82.471-37.002-82.471-82.473
		c0-32.46,19.193-62.005,48.889-75.279l7.095-3.171l-0.604-9.688c-0.122-1.839-0.265-3.672-0.265-5.543
		c0-66.74,54.298-121.041,121.038-121.041c66.749,0,121.055,54.296,121.036,121.114c-0.032,0.523-0.065,1.032-0.07,1.546
		l-0.132,13.691l13.444-2.578c5.313-1.021,10.594-1.531,15.701-1.531c45.479,0,82.472,36.995,82.472,82.479
		C431.018,338.303,394.011,375.305,348.533,375.305z"
        />
      </g>
      <g />
      <g />
      <g />
      <g />
      <g />
      <g />
      <g />
      <g />
      <g />
      <g />
      <g />
      <g />
      <g />
      <g />
      <g />
    </svg>
  </div>
</test>

<style>
  test {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    height: 100vh;
  }

  .btn:hover {
    opacity: 0.9;
  }

  .btn {
    width: 100%;
    margin-top: 0.5em;
    color: white;
    border: 0;
    border-radius: 2px;
    padding: 7px;
    font-weight: 200;
    font-size: 24px;

    background: #8e2de2; /* fallback for old browsers */
    background: -webkit-linear-gradient(
      to right,
      #4a00e0,
      #8e2de2
    ); /* Chrome 10-25, Safari 5.1-6 */
    background: linear-gradient(
      to right,
      #4a00e0,
      #8e2de2
    ); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
  }

  .login {
    width: 300px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  h1 {
    margin-bottom: 1em;
    font-size: 45px;
  }

  .top {
    height: 300px;
    width: 100%;
    clip-path: polygon(100% 88%, 100% 0%, 0% 0%);

    background: #8e2de2; /* fallback for old browsers */
    background: -webkit-linear-gradient(
      to left,
      #4a00e0,
      #8e2de2
    ); /* Chrome 10-25, Safari 5.1-6 */
    background: linear-gradient(
      to left,
      #4a00e0,
      #8e2de2
    ); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
  }

  .bottom {
    height: 300px;
    width: 100%;
    clip-path: polygon(0 15%, 0 100%, 100% 100%);

    background: #8e2de2; /* fallback for old browsers */
    background: -webkit-linear-gradient(
      to right,
      #4a00e0,
      #8e2de2
    ); /* Chrome 10-25, Safari 5.1-6 */
    background: linear-gradient(
      to right,
      #4a00e0,
      #8e2de2
    ); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
  }

  .parent {
    height: 400px;
    width: 400px;
    background-color: royalblue;
  }

  .child {
    background-color: red;
    height: 50%;
    width: 50%;
    transition: 1s cubic-bezier(0.165, 0.84, 0.44, 1);
  }

  .parent:hover .child {
    transform: translateX(100%);
  }
</style>
