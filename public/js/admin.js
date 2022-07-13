const form = document.querySelector('#adminLoginForm');

form.addEventListener('submit', async function (event) {
  // prevent page refresh on submit event
  // handling form with AJAX on frontend
  event.preventDefault();

  // strip form input values and put in an object
  const formData = new FormData(event.target);
  const formProps = Object.fromEntries(formData);

  try {
    const response = await fetch('/admin/login', {
      method: 'POST',
      redirect: 'follow',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(formProps),
    });
    if (response.ok) {
      return (window.location.href = '/admin');
    }

    // handle if status is not ok
  } catch (error) {
    // handle error
  }
});
