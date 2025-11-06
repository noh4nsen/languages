package com.example.controller;
import com.example.model.User;
import com.example.repository.UserRepository;
import io.micronaut.http.annotation.*;
import io.micronaut.http.HttpResponse;
import java.util.Optional;

@Controller("/users")
public class UserController {
    private final UserRepository repository;

    public UserController(UserRepository repository) {
        this.repository = repository;
    }

    @Get("/")
    public Iterable<User> list() { return repository.findAll(); }

    @Post("/")
    public HttpResponse<User> create(@Body User user) {
        User created = repository.save(user);
        return HttpResponse.created(created);
    }

    @Get("/{id}")
    public HttpResponse<User> get(Long id) {
        Optional<User> user = repository.findById(id);
        return user.map(HttpResponse::ok).orElseGet(HttpResponse::notFound);
    }

    @Put("/{id}")
    public HttpResponse<User> update(Long id, @Body User updated) {
        if (!repository.existsById(id)) return HttpResponse.notFound();
        updated.setId(id);
        return HttpResponse.ok(repository.update(updated));
    }

    @Delete("/{id}")
    public HttpResponse<?> delete(Long id) {
        repository.deleteById(id);
        return HttpResponse.noContent();
    }
}
