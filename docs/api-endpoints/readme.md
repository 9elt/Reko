[Docs](../) > [API endpoints](/)

# API endpoints

### User recommendations
<pre>
<a href="#user-recommendations"><b>POST</b></a> /recommendations/{my_anime_list_username}
</pre>

#### data:

*(optional)* **reload_list**: boolean  
> update the user's anime list  
> after 3 days are reloaded by default  

*(optional)* **anime_to_ignore**: number[ ] 
> provide an array of anime ids (myanimelist.net ids)  
> that will be excluded from recommendations

### User statistics
*requires authentication*
<pre>
<a href="#user-statistics"><b>GET</b></a> /stats/{my_anime_list_username}
</pre>

#### query parameters:

*(optional)* **reload_list**: boolean

### Authentication

<pre>
<a href="#user-statistics"><b>GET</b></a> /auth
</pre>
