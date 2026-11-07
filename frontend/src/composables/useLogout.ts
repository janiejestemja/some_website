export async function useLogout(session: any) {
  const request = await fetch("/logout", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
  });
  try {
    const response = await request.json();
    if (response.success === true) {
      session.log_out();
    } 
    console.log(response);
  } catch(error) {
    console.log("Current user not logged in.");
  }
}
