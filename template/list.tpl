<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>All users</title>
</head>
<body>
<p><h2><a href="/"> <=back </a></h2></p>
<p><h2>All users</h2></p>
<table border="1" width="100%">
    <tr>
        <th>Account</th>
        <th>Password</th>
        <th>Client name</th>
        <th>Client id</th>
        <th>Summer balance</th>
        <th>Available balance</th>
        <th>Last update time</th>
        <th/><th/><th/>
    </tr>

    {% for o in accounts %}
    <tr>
        <td>{{ o._account_id }}</td>
        <td>{{ o._password }}</td>
        <td>{{ o._client_name }}</td>
        <td>{{ o._client_id }}</td>
        <td>￥{{ o._sum_balance }}</td>
        <td>￥{{ o._available_balance }}</td>
        <td>{{ o._last_update_time }} </td>
        <td><a href="/account/delete?id={{ o._account_id }}">delete</a></td>
        <td><a href="/account/detail?id={{ o._account_id }}">detail</a></td>
        <td><a href="/account/refresh?id={{ o._account_id }}">refresh</a></td>
    </tr>
    {% endfor %}

</table>
</body>
</html>