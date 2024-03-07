document.addEventListener('DOMContentLoaded', function () {
  const searchForm = document.getElementById('search-form');
  const searchResults = document.getElementById('search-results');

  searchForm.addEventListener('submit', (event) => {
    event.preventDefault(); // Prevent default form submission

    // Get selected filter values
    const aptamerType = document.getElementById('m_AptamerType').value;
    const sortBy = document.getElementById('m_SortBy').value;
    const show = document.getElementById('m_Show').value;

    // Construct the query string with optional parameters
    const queryString = new URLSearchParams();
    if (aptamerType !== 'All') queryString.append('aptamerType', aptamerType);
    if (sortBy) queryString.append('sortBy', sortBy);
    if (show) queryString.append('show', show);
//  `https://aptabase.shuttleapp.rs/v1/fetch/${queryString.toString()}`
    fetch(`https://aptabase.shuttleapp.rs/v1/fetch`)
    .then(response => {
      if (!response.ok) {
        throw new Error('Network response was not ok');
      }
      return response.json();
    })
    .then(data => {
      // Clear existing results
      searchResults.innerHTML = '';

      // Create table element
      const table = document.createElement('table');
      table.classList.add('search-table');

      // Create table header row
      const headerRow = table.createTHead().insertRow();
      Object.keys(data[0]).forEach(key => {
        const headerCell = document.createElement('th');
        headerCell.textContent = key;
        headerRow.appendChild(headerCell);
      });

      // Create table body
      const tbody = table.createTBody();
      data.forEach(result => {
        const row = tbody.insertRow();
        Object.values(result).forEach(value => {
          const cell = row.insertCell();
          cell.textContent = value;
        });
      });

      // Append table to searchResults div
      searchResults.appendChild(table);
    })
    .catch(error => {
      console.error('Error fetching data:', error);
      // Handle errors appropriately
    });
  });
});
